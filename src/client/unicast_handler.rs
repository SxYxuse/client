/*use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use std::io::ErrorKind::WouldBlock;
use std::time::Duration;
use std::net::Shutdown;
use crate::builder::Builder;

pub struct UnicastHandler {
    address: String,
    builder: Builder,
}

impl UnicastHandler {
    pub fn new(ip_address: &str, port: u16) -> Self {
        UnicastHandler {
            address: format!("{}:{}", ip_address, port),
            builder: Builder,
        }
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub fn start_listening(&self) {
        let address = self.address.clone();
        let pool = ThreadPool::new(4);
        let listener = TcpListener::bind(&address).unwrap();
        listener.set_nonblocking(true).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let stream = Arc::new(Mutex::new(stream));
                    pool.execute(move || {
                        let mut stream = stream.lock().unwrap();
                        match Self::handle_client(&mut *stream) {
                            Ok(_) => (),
                            Err(e) => eprintln!("Erreur lors de la gestion du client: {}", e),
                        }
                        stream.shutdown(Shutdown::Both).unwrap();
                    });
                }
                Err(ref e) if e.kind() == WouldBlock => {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
                Err(e) => panic!("Erreur lors de l'acceptation de la connexion: {}", e),
            }
        }
    }

    fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
        let mut buffer = [0; 512];
        loop {
            let bytes_read = stream.read(&mut buffer)?;
            if bytes_read == 0 {
                return Ok(());
            }

            let message = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Message reçu du client: {}", message);

            let response = "Message bien reçu!";
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
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

    pub fn send_message(&self, message: &str) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(&self.address)?;
        stream.write_all(message.as_bytes())?;
        stream.flush()
    }
}*/

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use crate::builder::Builder;

pub struct UnicastHandler {
    listen_address: String,
    sender_address: String,
    builder: Builder,
}

impl UnicastHandler {
    pub fn new(ip_address: &str, listen_port: u16, sender_port: u16) -> Self {
        UnicastHandler {
            listen_address: format!("{}:{}", ip_address, listen_port),
            sender_address: format!("{}:{}", ip_address, sender_port),
            builder: Builder,
        }
    }

    pub fn get_address(&self) -> String {
        self.listen_address.clone()
    }

    pub fn start_listening(&self) {
        let address = self.listen_address.clone();
        thread::spawn(move || {
            loop {
                if let Err(e) = Self::listen_for_messages(&address) {
                    eprintln!("Erreur lors de l'écoute des messages: {}", e);
                }
            }
        });
    }

    // RECEIVE MESSAGE
    fn listen_for_messages(address: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(err) = Self::handle_client(stream) {
                        eprintln!("Erreur lors de la gestion du client: {}", err);
                    }
                }
                Err(e) => {
                    eprintln!("Erreur lors de l'acceptation de la connexion: {}", e);
                }
            }
        }
        Ok(())
    }
//    // stop the listening thread
//    pub fn stop_listening(&self) {
//        let _ = self.stop_sender.send(());
//    }

    fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
        let mut buffer = [0; 512];
        loop {
            let bytes_read = stream.read(&mut buffer)?;
            if bytes_read == 0 {
                return Ok(());
            }

            let message = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Message reçu du client: {}", message);

            let response = "Message bien reçu!";
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
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

    pub fn send_message(&self, message: &str) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(&self.sender_address)?;
        stream.write_all(message.as_bytes())?;
        stream.flush()
    }
}