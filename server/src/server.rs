use core::panic;
use log::{debug, info};
use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::thread;

use shared::shared::{InputType, MsgContentTypes, RequestType};

static INIT_LOGGER: Once = ONCE_INIT;
static LOGGER_INITIALIZED: Mutex<bool> = Mutex::new(false);

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum RequestResult {
    Parsed(RequestType, String),
    Error(io::Error),
}

// TODO understand this
pub fn init_logger() {
    INIT_LOGGER.call_once(|| {
        let mut initialized = LOGGER_INITIALIZED.lock().unwrap();

        if !*initialized {
            env_logger::init();
            *initialized = true;
        }
    });
}

// worker thread requests?
struct Worker {
    id: usize,
    shared_requests: Arc<Mutex<Vec<TcpStream>>>,
}

impl Worker {
    fn new(id: usize, shared_requests: Arc<Mutex<Vec<TcpStream>>>) -> Self {
        Worker {
            id,
            shared_requests,
        }
    }

    fn start(self) {
        let shared_requests_clone = Arc::clone(&self.shared_requests);
        info!("starting worker thread: {}", self.id);

        thread::spawn(move || loop {
            info!("starting worker thread: {} looping", self.id);
            let request = {
                let mut requests = shared_requests_clone.lock().unwrap();
                if let Some(stream) = requests.pop() {
                    info!("starting worker thread: {} looping found stream", self.id);
                    stream
                } else {
                    break;
                }
            };

            info!(
                "starting worker thread: {} calling process_request",
                self.id
            );
            self.process_request(request);
        });
    }

    fn process_request(&self, stream: TcpStream) {
        println!(
            "Worker {} processing a request from {:?}",
            self.id,
            stream.peer_addr().unwrap()
        );
    }
}

pub struct Server {
    input_type: InputType,
    port: u32,
}

impl Server {
    pub fn new(input_type: InputType, port: u32) -> Self {
        Server { input_type, port }
    }

    pub fn start(&self) -> io::Result<()> {
        init_logger();

        let shared_requests: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

        let mut worker_handles = vec![];

        for worker_id in 0..1 {
            let shared_requests_clone = Arc::clone(&shared_requests);
            let worker = Worker::new(worker_id, shared_requests_clone);
            worker_handles.push(thread::spawn(move || worker.start()));
        }

        info!("server BEGIN");

        let address = format!("127.0.0.1:{}", self.port);

        let listener = TcpListener::bind(address)?;

        info!("binded to port ");

        // check if stream contains a sync request or async request
        // if sync: use blocking and return a response
        // else: idk
        for stream in listener.incoming() {
            //info!("adding to list");
            //shared_requests.lock().unwrap().push(stream?);
            //info!("added to list");

            // TODO do handle in a separate thread from a pool
            let stream = stream?;

            // duplicating stream for write. TODO try blocking the other action in both streams?
            let mut write_stream = stream.try_clone().expect("Failed to clone TcpStream");

            let request = self.parse_request(stream);

            match request {
                RequestResult::Parsed(request_type, request) => {
                    info!(
                        "Parsed request with type: {:#?} and request: {}",
                        request_type, request
                    );

                    let response = self.handle_request(request);

                    match response {
                        Ok(res) => {
                            info!("Found response: {}", res);

                            match request_type {
                                RequestType::SYNC => {
                                    info!("Found result. Writing back {}", res);

                                    if res == "KILL" {
                                        info!("Received KILL cmd, killing");
                                        break;
                                    }

                                    write_stream.write_all(res.as_bytes())?;
                                }
                                RequestType::ASYNC => {
                                    info!("Found result {}, but not sending response back", res);
                                }
                            }
                        }
                        Err(err) => info!("{}", err),
                    }
                }
                RequestResult::Error(err) => {
                    info!("Found err: {}", err);
                }
            }
        }

        drop(shared_requests);

        for handle in worker_handles {
            handle.join().unwrap();
        }

        Ok(())
    }

    fn parse_request(&self, mut stream: TcpStream) -> RequestResult {
        debug!("parse_request BEGIN msg_type: {:#?}", self.input_type);

        let mut buffer = [0; 512];

        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let request_str = String::from_utf8_lossy(&buffer[..]).into_owned();

                debug!("parse_request request: {:#?}", request_str);

                if request_str.starts_with("SYNC;") {
                    RequestResult::Parsed(
                        RequestType::SYNC,
                        String::from(request_str.split_at(5).1),
                    )
                } else if request_str.starts_with("ASYNC;") {
                    RequestResult::Parsed(
                        RequestType::ASYNC,
                        String::from(request_str.split_at(6).1),
                    )
                } else {
                    RequestResult::Error(io::Error::new(
                        io::ErrorKind::Other,
                        "Could not find SYNC or ASYNC",
                    ))
                }
            }
            Err(err) => RequestResult::Error(err),
        }
    }

    // main func that parses the request and makes the procedure call
    fn handle_request(&self, request: String) -> Result<String, String> {
        info!("handle_client BEGIN msg_type: {:#?}", self.input_type);

        let res = match self.input_type {
            InputType::STR => self.evaluate_expr(&request),
            InputType::JSON => self.evaluate_expr_json(&request),
        };

        res

        //Ok(HandleResult::Normal)
    }

    fn evaluate_expr_json(&self, expression: &str) -> Result<String, String> {
        // TODO why is this needed?
        let expression = expression.trim().trim_matches('\0');

        info!("evaluate_expr_json BEGIN {:#?}", expression);

        let deserialized: MsgContentTypes = serde_json::from_str(&expression).unwrap();

        let expr = match deserialized {
            MsgContentTypes::Type1(operation_struct) => operation_struct,
            MsgContentTypes::Type2(_) => {
                // TODO understand the cmd
                return Ok(format!("KILL"));
            }
        };

        let op_char = expr.operator.chars().next().unwrap();
        //// TODO make this in below function too
        let op: Operation = match op_char {
            '*' => Operation::Multiply,
            '+' => Operation::Add,
            '-' => Operation::Subtract,
            '/' => Operation::Divide,
            _ => return Err(format!("Operand {:#?} is not supported", op_char)),
        };

        return Ok(evaluate(
            &expr.operand1.to_string(),
            &expr.operand2.to_string(),
            op,
        ));
    }

    // TODO call diff func on json?
    fn evaluate_expr(&self, expression: &str) -> Result<String, String> {
        let expression = expression.trim().trim_matches('\0');

        if expression == "KILL" {
            return Ok(format!("KILL"));
        } else if let Some(index) = expression.find(|c: char| !c.is_numeric()) {
            debug!("{:#?}", index);
            let (operand1, operand2) = expression.split_at(index);

            let op = operand2.chars().next().unwrap();

            let operand2 = &operand2[1..];

            debug!("{:#?}", expression.chars().nth(index));

            // TODO make this in below function too
            let op: Operation = match op {
                '*' => Operation::Multiply,
                '+' => Operation::Add,
                '-' => Operation::Subtract,
                '/' => Operation::Divide,
                _ => return Err(format!("Operand {:#?} is not supported", op)),
            };

            return Ok(evaluate(operand1, operand2, op));
        }

        Err(format!("Final return Invalid expr. Expect <opr1>+<opr2>"))
    }
}

fn evaluate(operand1: &str, operand2: &str, operation: Operation) -> String {
    let (operand1, operand2) = (
        operand1.parse::<i32>().unwrap(),
        operand2.parse::<i32>().unwrap(),
    );

    info!(
        "Operating {:#?} {:#?} with operand {:#?}",
        operand1, operand2, operation
    );

    let result = match operation {
        Operation::Add => Ok(operand1 + operand2),
        Operation::Subtract => Ok(operand1 - operand2),
        Operation::Multiply => Ok(operand1 * operand2),
        Operation::Divide => match operand2 {
            0 => Err("Division by zero"),
            _ => Ok(operand1 / operand2),
        },
    };

    match result {
        Ok(ans) => ans.to_string(),
        Err(err) => err.to_string(),
    }
}
