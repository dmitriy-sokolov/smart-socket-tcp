use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

use crate::command::Command;
use crate::response::Response;

pub struct SocketClient {
    stream: TcpStream,
}

impl SocketClient {
    pub fn new(server_address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(server_address)?;
        Ok(Self { stream })
    }

    pub fn run_command(&mut self, command: Command) -> Result<Response, Box<dyn Error>> {
        self.stream.write_all(&[command.into()])?;
        let mut buffer = [0u8; 5];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer.into())
    }
}
