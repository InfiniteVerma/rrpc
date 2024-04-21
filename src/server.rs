use core::panic;
use log::{debug, info};
use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn init() {
    env_logger::init();
}

pub fn start() -> io::Result<()> {
    init();

    debug!("server BEGIN");

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    info!("binded to port ");

    for stream in listener.incoming() {
        let stream = stream?;
        handle_client(stream)?;
    }

    Ok(())
}

// main func that parses the request and makes the procedure call
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    debug!("handle_client BEGIN");

    let mut buffer = [0; 512];

    stream.read(&mut buffer)?;

    let expr = String::from_utf8_lossy(&buffer[..]);

    let result = evaluate_expr(&expr);

    stream.write_all(result.as_bytes())?;

    Ok(())
}

fn evaluate_expr(expression: &str) -> String {
    let expression = expression.trim().trim_matches('\0');

    if let Some(index) = expression.find(|c: char| !c.is_numeric()) {
        debug!("{:#?}", index);
        let (operand1, operand2) = expression.split_at(index);

        let op = operand2.chars().next().unwrap();

        let operand2 = &operand2[1..];

        debug!("{:#?}", expression.chars().nth(index));

        let op: Operation = match op {
            '*' => Operation::Multiply,
            '+' => Operation::Add,
            '-' => Operation::Subtract,
            '/' => Operation::Divide,
            _ => panic!("Operand {:#?} is not supported", op),
        };

        return evaluate(operand1, operand2, op);
    }

    String::from("Final return Invalid expr. Expect <opr1>+<opr2>")
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
