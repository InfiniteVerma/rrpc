use log::{debug, info};
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpStream};

use shared::shared::{InputType, KILL_JSON, KILL_STR};

pub fn init() {
    env_logger::init();
}

pub struct Client {
    input_type: InputType,
    port: u32, // TODO validate port range
}

impl Client {
    /// Returns an instance of Client
    pub fn new(input_type: InputType, port: u32) -> Self {
        Client { input_type, port }
    }

    // TODO pass port as variable
    /// Connects to a server. TODO Isn't used?
    pub fn connect() -> io::Result<TcpStream> {
        TcpStream::connect("127.0.0.1:8080")
    }

    /// Sends a KILL command to stop the server
    pub fn kill(&self) -> io::Result<()> {
        debug!("send BEGIN");

        match self.input_type {
            InputType::STR => self.send_sync(KILL_STR).unwrap(),
            InputType::JSON => self.send_sync(KILL_JSON).unwrap(),
        };

        Ok(())
    }

    /// Sends a SYNC command to the server
    ///
    /// This is a blocking call and waits for a response from the server
    pub fn send_sync(&self, expression: &str) -> io::Result<String> {
        debug!("send_sync BEGIN");

        let address = format!("127.0.0.1:{}", self.port);

        let mut stream = TcpStream::connect(address)?;

        // add SYNC as prefix to the expression
        let expression = match self.input_type {
            InputType::STR => format!("SYNC;{}", expression),
            InputType::JSON => format!("{}", expression), // TODO
        };

        debug!("connected, writing expression to socket: {}", expression);

        stream.write_all(expression.as_bytes())?;

        let mut response = String::new();

        // TODO this will change based on string/JSON
        stream.read_to_string(&mut response)?;

        debug!("received response: {}", response);

        // TODO is below needed?
        //stream
        //    .shutdown(Shutdown::Both)
        //    .expect("shutdown call failed");
        Ok(response)
    }

    /// Sends a ASYNC command to the server
    ///
    /// This is a nonblocking call. Client doesn't wait for a response.
    pub fn send_async(&self, expression: &str) -> Result<(), io::Error> {
        debug!("send_async BEGIN");

        let address = format!("127.0.0.1:{}", self.port);

        let mut stream = TcpStream::connect(address)?;

        // add ASYNC as prefix to the expression
        let expression = match self.input_type {
            InputType::STR => format!("ASYNC;{}", expression),
            InputType::JSON => format!("{}", expression), // TODO
        };

        stream.write_all(expression.as_bytes())?;

        Ok(())
    }
}
