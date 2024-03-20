use std::net::{TcpStream};
use std::io::{Read, Write};
use crate::builder::Builder;

pub struct UnicastHandler {
    sender_address: String,
    builder: Builder,
}

impl UnicastHandler {
    pub fn new(ip_address: &str, sender_port: u16) -> Self {
        UnicastHandler {
            sender_address: format!("{}:{}", ip_address, sender_port),
            builder: Builder,
        }
    }

    //SEND MESSAGE
    pub fn build_newmon_message(&self, augmented_url: &str) -> String {
        self.builder.build_newmon_message(augmented_url)
    }

    pub fn build_listmon_message(&self) -> String {
        self.builder.build_listmon_message()
    }

    pub fn build_request_message(&self, id: &str) -> String {
        self.builder.build_request_message(id)
    }

    pub fn send_message(&self, message: &str) -> std::io::Result<String> {
        let mut stream = TcpStream::connect(&self.sender_address)?;
        stream.write_all(message.as_bytes())?;
        stream.flush()?;

        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
        Ok(response)
    }
}