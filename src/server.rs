use std::net::{TcpListener, TcpStream};
use std::io::{self, prelude::*};
use log::info;

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

    let parts: Vec<&str> = expression.trim().trim_matches('\0').split('+').collect();
    
    if parts.len() != 2 {
        return String::from("Invalid expr. Expect <opr1>+<opr2>");
    }

    if let (Ok(opr1), Ok(opr2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
        let result = opr1 + opr2;
        return result.to_string();
    }

    String::from("Final return Invalid expr. Expect <opr1>+<opr2>")
}
