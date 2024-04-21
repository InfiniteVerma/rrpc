use std::net::TcpStream;
use std::io::{self, Read, Write};
use log::info;

pub fn init() {
    env_logger::init();
}

// TODO pass port as variable
pub fn connect() -> io::Result<TcpStream> {
    TcpStream::connect("127.0.0.1:8080")
}

pub fn send(expression: &str) -> io::Result<String> {
    info!("send BEGIN");

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    info!("connected, writing expression to socket: {}", expression);
    stream.write_all(expression.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    info!("received response: {}", response);
    Ok(response)
}
