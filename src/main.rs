use crate::client::unicast_handler;

mod tests;
mod analyzer;
mod builder;
mod client;

use std::io::{self, Write};
use crate::client::unicast_handler::UnicastHandler;

fn main() {
    let handler = UnicastHandler::new("192.168.133.2", 60001);

    loop {
        let mut input = String::new();
        println!("Menu:");
        println!("1. Add service");
        println!("2. List services");
        println!("3. Check status");
        println!("4. Quit");
        print!("Enter command: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let command: i32 = input.trim().parse().unwrap_or(0);
                match command {
                    1 => {
                        print!("Enter augmented_url: ");
                        io::stdout().flush().unwrap();
                        let mut url = String::new();
                        match io::stdin().read_line(&mut url) {
                            Ok(_) => {
                                let message = handler.build_newmon_message(url.trim());
                                let response = handler.send_message(&message).unwrap();
                                print!("Response from monitor: {}", response)
                            }
                            Err(e) => {
                                eprintln!("Erreur lors de la lecture de l'entrée de l'utilisateur: {}", e);
                                continue;
                            }
                        }
                    }
                    2 => {
                        let message = handler.build_listmon_message();
                        let response = handler.send_message(&message).unwrap();
                        println!("Response from monitor: {}", response);
                    }
                    3 => {
                        print!("Enter id: ");
                        io::stdout().flush().unwrap();
                        let mut id = String::new();
                        match io::stdin().read_line(&mut id) {
                            Ok(_) => {
                                let message = handler.build_request_message(id.trim());
                                let response = handler.send_message(&message).unwrap();
                                println!("Response from monitor: {}", response)
                            }
                            Err(e) => {
                                eprintln!("Erreur lors de la lecture de l'entrée de l'utilisateur: {}", e);
                                continue;
                            }
                        }
                    }
                    4 => {
                        println!("Exiting program.");
                        break;
                    }
                    _ => println!("Unknown command"),
                }
            }
            Err(e) => {
                eprintln!("Erreur lors de la lecture de l'entrée de l'utilisateur: {}", e);
                continue;
            }
        }
    }
}
