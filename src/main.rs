use crate::client::unicast_handler;

mod tests;
mod analyzer;
mod builder;
mod client;

use std::io::{self, Write};
use crate::client::unicast_handler::UnicastHandler;

fn main() {
    let handler = UnicastHandler::new("192.168.133.5", 60001);

    loop {
        let mut input = String::new();
        println!("Menu:");
        println!("1. Add service....:  NEWMON <augmented_url>");
        println!("2. List services..:  LISTMON");
        println!("3. Check status...:  REQUEST <id>");
        println!("4. Quit...........:  QUIT");
        print!("Enter command: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }
                //PRENDRE EN COMPTE LES MINISCULE A L ENTRE DE L UTILISATEUR
                match parts[0] {
                    "NEWMON" => {
                        if parts.len() < 2 {
                            println!("Usage: NEWMON <augmented_url>");
                            continue;
                        }
                        let message = handler.build_newmon_message(parts[1]);
                        let reponse = handler.send_message(&message).unwrap();
                        print!("Response from monitor: {}", reponse)
                    }
                    "LISTMON" => {
                        let message = handler.build_listmon_message();
                        let response = handler.send_message(&message).unwrap();
                        println!("Response from monitor: {}", response);
                    }
                    "REQUEST" => {
                        if parts.len() < 2 {
                            println!("Usage: REQUEST <id>");
                            continue;
                        }
                        let message = handler.build_request_message(parts[1]);
                        let reponse = handler.send_message(&message).unwrap();
                        println!("Response from monitor: {}", reponse)
                    }
                    "QUIT" => {
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
