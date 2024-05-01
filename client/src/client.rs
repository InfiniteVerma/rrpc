use log::{debug, info};
use std::io::{self, Read, Write};
use std::net::TcpStream;

use shared::shared::{InputType, KILL_JSON, KILL_STR};

pub fn init() {
    env_logger::init();
}

pub struct Client {
    input_type: InputType,
    port: u32, // TODO validate port range
}

impl Client {
    pub fn new(input_type: InputType, port: u32) -> Self {
        Client { input_type, port }
    }

    // TODO pass port as variable
    pub fn connect() -> io::Result<TcpStream> {
        TcpStream::connect("127.0.0.1:8080")
    }

    pub fn kill(&self) -> io::Result<()> {
        debug!("send BEGIN");

        match self.input_type {
            InputType::STR => self.send_sync(KILL_STR).unwrap(),
            InputType::JSON => self.send_sync(KILL_JSON).unwrap(),
        };

        Ok(())
    }

    pub fn send_sync(&self, expression: &str) -> io::Result<String> {
        debug!("send BEGIN");

        let address = format!("127.0.0.1:{}", self.port);

        let mut stream = TcpStream::connect(address)?;

        debug!("connected, writing expression to socket: {}", expression);
        stream.write_all(expression.as_bytes())?;

        let mut response = String::new();

        // TODO this will change based on string/JSON
        stream.read_to_string(&mut response)?;

        debug!("received response: {}", response);
        Ok(response)
    }
}
