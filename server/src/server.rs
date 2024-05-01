use log::{debug, info, log};
use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::sync::{Mutex, Once, ONCE_INIT};
use std::thread::sleep;
use std::time::Duration;

use shared::shared::{InputType, MsgContentTypes};

static INIT_LOGGER: Once = ONCE_INIT;
static LOGGER_INITIALIZED: Mutex<bool> = Mutex::new(false);

const MAX_RETRIES: usize = 5;
const RETRY_INTERVAL_MS: u64 = 1000;

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum HandleResult {
    Normal,
    Kill,
}

pub fn init_logger() {
    INIT_LOGGER.call_once(|| {
        let mut initialized = LOGGER_INITIALIZED.lock().unwrap();

        if !*initialized {
            env_logger::init();
            *initialized = true;
        }
    });
}

pub struct Server {
    input_type: InputType,
    port: u32,
}

impl Server {
    pub fn new(input_type: InputType, port: u32) -> Self {
        Server { input_type, port }
    }

    fn try_bind(&self, address: &str) -> Option<TcpListener> {
        for attempt in 0..MAX_RETRIES {
            match TcpListener::bind(address) {
                Ok(listener) => {
                    info!("Successfully bound to port {} attempt: {}", address, attempt);
                    Some(listener);
                }
                Err(err) => {
                    info!("Failed to bind to port {}", address);
                    if attempt < MAX_RETRIES - 1{
                        info!("Retrying in {} ms ...", RETRY_INTERVAL_MS);
                        sleep(Duration::from_millis(RETRY_INTERVAL_MS));
                    }
                }
            }
        }
        None
    }

    pub fn start(&self) -> io::Result<()> {
        // TODO find a cleaner way to initialize this
        init_logger();

        info!("server BEGIN");

        let address = format!("127.0.0.1:{}", self.port);

        let listener = TcpListener::bind(address)?;
        //let listener = self.try_bind(&address).unwrap();

        info!("binded to port ");

        for stream in listener.incoming() {
            let stream = stream?;
            // TODO do handle in a separate thread from a pool
            match self.handle_client(stream) {
                Ok(HandleResult::Normal) => continue,
                Ok(HandleResult::Kill) => {
                    info!("Received KILL cmd, killing");
                    break;
                }
                Err(err_msg) => {
                    info!(
                        "Error while evaluating, continuing for next requests. Msg: {}",
                        err_msg
                    );
                }
            }
        }

        Ok(())
    }

    // main func that parses the request and makes the procedure call
    fn handle_client(&self, mut stream: TcpStream) -> io::Result<HandleResult> {
        info!("handle_client BEGIN msg_type: {:#?}", self.input_type);

        let mut buffer = [0; 512];

        stream.read(&mut buffer)?;

        let res = match self.input_type {
            InputType::STR => {
                let str_expr = String::from_utf8_lossy(&buffer[..]);
                self.evaluate_expr(&str_expr)
            }
            InputType::JSON => {
                let str_expr = String::from_utf8_lossy(&buffer[..]);
                self.evaluate_expr_json(&str_expr)
            }
        };

        match res {
            Ok(result) => {
                info!("Found result. Writing back {}", result);
                stream.write_all(result.as_bytes())?;

                if result == "KILL" {
                    return Ok(HandleResult::Kill);
                }
            }
            Err(err_msg) => {
                return Err(io::Error::new(io::ErrorKind::Other, err_msg));
            }
        }

        Ok(HandleResult::Normal)
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
