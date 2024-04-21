use std::net::TcpStream;
use std::io::{self};

// TODO pass port as variable
pub fn connect() -> io::Result<TcpStream> {
    TcpStream::connect("127.0.0.1:8080")
}
