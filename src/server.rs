use std::net::{TcpListener, TcpStream};
use std::io::{self};

pub fn start() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let stream = stream?;
        handle_client(stream)?;
    }

    Ok(())
}

fn handle_client(_stream: TcpStream) -> io::Result<()> {
    Ok(())
}
