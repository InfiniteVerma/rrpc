use core::panic;
use log::{debug, info};
use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::thread;

use shared::shared::{Calculation, Command, InputType, MsgContentTypes, RequestType};

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
    ParsedStr(String),
    ParsedStrCommand(String),
    ParsedJSONCalculation(Calculation),
    ParsedJSONCommand(Command),
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
    shared_requests: Arc<Mutex<Vec<(RequestType, RequestResult)>>>,
}

impl Worker {
    fn new(id: usize, shared_requests: Arc<Mutex<Vec<(RequestType, RequestResult)>>>) -> Self {
        Worker {
            id,
            shared_requests,
        }
    }

    fn start(self) {
        let shared_requests_clone = Arc::clone(&self.shared_requests);
        debug!("starting worker thread: {}", self.id);

        thread::spawn(move || loop {
            debug!("starting worker thread: {} looping", self.id);
            let mut requests = shared_requests_clone.lock().unwrap();

            if requests.len() != 0 {
                info!("Worker {} requests.size(): {}", self.id, requests.len());
            }

            let request = requests.pop();
            match request {
                Some(req) => {
                    info!("Worker {} executing request", self.id);
                    self.process_request(req);
                }
                idk => {
                    debug!(
                        "Worker {} Didn't find Some, continuing the loop {:#?}",
                        self.id, idk
                    );
                }
            }
        });
    }

    // TODO do this first
    fn process_request(&self, (request_type, request_result): (RequestType, RequestResult)) {
        info!(
            "Worker {} processing a request type {:?}",
            self.id, request_type
        );

        match request_type {
            RequestType::SYNC => panic!("Worker thread doesn't do sync request TODO"),
            RequestType::ASYNC => match request_result {
                RequestResult::ParsedStr(req) => {
                    info!("ASYNC ParsedStr Parsed request with request: {}", req);

                    let response = evaluate_expr(&req);

                    info!("ASYNC evaluate_expr returns: {:#?}", response);
                }
                RequestResult::ParsedStrCommand(req) => {
                    info!(
                        "ASYNC ParsedStrCommand  Parsed request with request: {}",
                        req
                    );
                }
                RequestResult::ParsedJSONCommand(req) => {
                    panic!("ParsedStr not handled in ASYNC context");
                }
                RequestResult::ParsedJSONCalculation(req) => {
                    info!("ASYNC Parsed json request: {:#?}", req);

                    let response = evaluate_expr_json(&req);

                    info!("ASYNC evaluate_expr_json returns: {:#?}", response);
                }
                RequestResult::Error(err) => {
                    info!("{}", err);
                }
            },
        }
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
        info!("server BEGIN");

        init_logger();

        let shared_requests: Arc<Mutex<Vec<(RequestType, RequestResult)>>> =
            Arc::new(Mutex::new(Vec::new()));

        let mut worker_handles = vec![];

        info!("spawning worker threads");

        for worker_id in 0..1 {
            let shared_requests_clone = Arc::clone(&shared_requests);
            let worker = Worker::new(worker_id, shared_requests_clone);
            worker_handles.push(thread::spawn(move || worker.start()));
        }

        let address = format!("127.0.0.1:{}", self.port);

        let listener = TcpListener::bind(address)?;
        info!("binded to port ");

        // check if stream contains a sync request or async request
        // if sync: use blocking and return a response
        // else: idk
        for stream in listener.incoming() {
            info!("found new stream");

            let stream = stream?;

            let mut write_stream = stream.try_clone().expect("Failed to clone TcpStream");

            // if async, send the message to list
            // duplicating stream for write. TODO try blocking the other action in both streams?
            let (request_type, request_result) = self.parse_request(stream);

            // else, execute and return here only
            let response: Result<String, String> = match request_type {
                RequestType::SYNC => {
                    // if command == KILL == break
                    // else execute evaluate_expr
                    let response = match request_result {
                        RequestResult::ParsedStrCommand(cmd) => {
                            info!("Parsed request with request: {}", cmd);
                            break;
                        }
                        RequestResult::ParsedStr(request) => {
                            info!("Parsed request with request: {}", request);

                            let response = evaluate_expr(&request);

                            info!("evaluate_expr returns: {:#?}", response);

                            response
                        }
                        RequestResult::ParsedJSONCalculation(request) => {
                            info!("Parsed json request: {:#?}", request);

                            let response = evaluate_expr_json(&request);

                            response
                        }
                        RequestResult::ParsedJSONCommand(request) => {
                            info!("Got a command. Killing for now! TODO ");
                            break;
                        }
                        RequestResult::Error(err) => {
                            info!("Found err: {}", err);
                            continue;
                        }
                    };

                    response
                }
                RequestType::ASYNC => {
                    shared_requests
                        .lock()
                        .unwrap()
                        .push((request_type, request_result));
                    continue;
                }
            };

            match response {
                Ok(res) => {
                    info!("Found response: {} sending it back", res);
                    write_stream.write_all(res.as_bytes())?;
                    debug!("write done");
                }
                Err(err) => {
                    info!("Found err: {}", err);
                    continue;
                }
            }
        }

        drop(shared_requests);

        for handle in worker_handles {
            handle.join().unwrap();
        }

        Ok(())
    }

    fn parse_request(&self, mut stream: TcpStream) -> (RequestType, RequestResult) {
        debug!("parse_request BEGIN msg_type: {:#?}", self.input_type);

        let mut buffer = [0; 512];

        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                debug!("Bytes read: {}", bytes_read);

                let request_str = String::from_utf8_lossy(&buffer[..]).into_owned();

                debug!("parse_request request: {:#?}", request_str);

                match self.input_type {
                    InputType::STR => {
                        if request_str.starts_with("SYNC;KILL") {
                            (
                                RequestType::SYNC,
                                RequestResult::ParsedStrCommand(String::from(
                                    request_str.split_at(5).1,
                                )),
                            )
                        } else if request_str.starts_with("SYNC;") {
                            (
                                RequestType::SYNC,
                                RequestResult::ParsedStr(String::from(request_str.split_at(5).1)),
                            )
                        } else if request_str.starts_with("ASYNC;") {
                            (
                                RequestType::ASYNC,
                                RequestResult::ParsedStr(String::from(request_str.split_at(6).1)),
                            )
                        } else {
                            // TODO?
                            (
                                RequestType::SYNC,
                                RequestResult::Error(io::Error::new(
                                    io::ErrorKind::Other,
                                    "Could not find SYNC or ASYNC",
                                )),
                            )
                        }
                    }
                    InputType::JSON => {
                        let request = request_str.trim().trim_matches('\0');

                        debug!("parse_request JSON {:#?}", request);

                        let deserialized: MsgContentTypes = serde_json::from_str(&request).unwrap();

                        debug!("Deserialized: deserialized: {:#?}", deserialized);

                        return match deserialized {
                            MsgContentTypes::Type1(op) => (
                                op.request_type.clone(),
                                RequestResult::ParsedJSONCalculation(op),
                            ),
                            MsgContentTypes::Type2(cmd) => (
                                cmd.request_type.clone(),
                                RequestResult::ParsedJSONCommand(cmd),
                            ),
                        };
                    }
                }
            }
            Err(err) => (RequestType::SYNC, RequestResult::Error(err)),
        }
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

fn evaluate_expr_json(expr: &Calculation) -> Result<String, String> {
    // TODO why is this needed?
    //let expression = expression.trim().trim_matches('\0');

    info!("evaluate_expr_json BEGIN {:#?}", expr);

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
fn evaluate_expr(expression: &str) -> Result<String, String> {
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
