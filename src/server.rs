use std::net::{TcpListener, TcpStream};
use std::io::{self, prelude::*};
use log::info;

enum Operation {
    Add,
    Subtract,
}

pub fn init() {
    env_logger::init();
}

pub fn start() -> io::Result<()> {
    init();

    info!("server BEGIN");

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
    info!("handle_client BEGIN");

    let mut buffer = [0; 512];

    stream.read(&mut buffer)?;

    let expr = String::from_utf8_lossy(&buffer[..]);

    let result = evaluate_expr(&expr);

    stream.write_all(result.as_bytes())?;
    
    Ok(())
}

fn evaluate_expr(expression: &str) -> String {

    let expression = expression.trim().trim_matches('\0');
    
    info!("{:#?}", expression);

    if let Some(index) = expression.find(|c: char| !c.is_numeric()) {
        info!("{:#?}", index);
        let (operand1, operand2) = expression.split_at(index);

        // TODO use enum instead of hardcoding Add
        return evaluate(operand1, operand2, Operation::Add);
    }

    String::from("Final return Invalid expr. Expect <opr1>+<opr2>")
}

fn evaluate(operand1: &str, operand2: &str, operation: Operation) -> String {

    let (operand1, operand2) = (operand1.parse::<i32>().unwrap(), operand2.parse::<i32>().unwrap());
    let result = operand1 + operand2;
    return result.to_string();
}
