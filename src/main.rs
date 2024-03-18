use crate::client::unicast_handler;

mod tests;
mod analyzer;
mod builder;
mod client;

use std::io::{self, Write};
use crate::client::unicast_handler::UnicastHandler;

fn main() {
    let handler = UnicastHandler::new("localhost", 20000, 12345);
    println!("Client ready to connect to {}", handler.get_address());
    handler.start_listening();
    println!("Server listening on {}", handler.get_address());

    loop {
        let mut input = String::new();
        println!("Menu:");
        println!("1. Add service....:  NEWMON <augmented_url>");
        println!("2. List services..:  LISTMON");
        println!("3. Check status...:  REQUEST <id>");
        println!("4. Quit...........:  QUIT");
        print!("Enter command: ");
        io::stdout().flush().unwrap(); // flush to ensure 'Enter command' is printed before blocking for input
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
                        handler.send_message(&message).unwrap();
                    }
                    "LISTMON" => {
                        let message = handler.build_listmon_message();
                        handler.send_message(&message).unwrap();
                    }
                    "REQUEST" => {
                        if parts.len() < 2 {
                            println!("Usage: REQUEST <id>");
                            continue;
                        }
                        let message = handler.build_request_message(parts[1]);
                        handler.send_message(&message).unwrap();
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
