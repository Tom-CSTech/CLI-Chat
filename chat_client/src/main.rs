#[allow(unreachable_code)]

use std::io;
use std::net::TcpStream;
use std::io::prelude::*;
//use std::str;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub user_id: usize,
    pub chat_id: usize,
    pub contents: String
}

fn main() {

    loop {

        let addr = String::from("127.0.0.1");
        let port = String::from("7878");

        let ip = format!("{}:{}", addr.trim(), port.trim());
        println!("IP: {}", ip);
        
        print!("User ID: ");
        // Flush stdout because
        io::stdout().flush().unwrap();
        let mut user_id = String::new();
        io::stdin().read_line(&mut user_id).unwrap();
        let user_id: usize = user_id.trim().parse().unwrap();

        match TcpStream::connect(ip) {
            Ok(stream) => {handle_connection(stream, user_id);},
            Err(e) => {eprintln!("Invalid input: \n{}", e); continue;}
        }
    }
}

/// Takes the incoming HTTP request and passes it through the internal 
/// functionality from the highest level.
fn handle_connection(mut stream: TcpStream, user_id: usize) {
    println!("Connection successful!");
    // Reset terminal once entering chat
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
            .args(&["/C","cls"])
            .status().expect("Failed to execute process (CLS)");
    #[cfg(target_os = "linux")]
    std::process::Command::new("sh")
            .args(&["-c","clear"])
            .output().expect("Failed to execute process (CLS)");
    
    // Loop for sequential requests
    loop {

        let chat_id = 50;
        let mut contents = String::new();
        println!("\nEnter a message: ");
        io::stdin().read_line(&mut contents).unwrap();

        let new_message = Message {
            user_id,
            chat_id,
            contents
        };

        let message_encoded = bincode::serialize(&new_message).unwrap();

        println!("Before stream write");

        match stream.write(&message_encoded) {
            Ok(_) => {
                println!("Stream write success");
                stream.flush().unwrap();
                let mut answer = [0; 8192];
                if let Ok(_) = stream.read(&mut answer) {
                    let answer: Vec<Message> = bincode::deserialize(&answer).unwrap();
                    // Reset terminal before displaying
                    #[cfg(target_os = "windows")]
                    std::process::Command::new("cmd").args(&["/C","cls"]).status().expect("Failed to execute process (CLS)");
                    #[cfg(target_os = "linux")]
                    std::process::Command::new("sh").args(&["-c","clear"]).output().expect("Failed to execute process (CLS)");
                    for message in answer {
                        if message.user_id == user_id {
                            println!("\t\t----------\n\t\tYou:\n\t\t{}\n\t\t----------", message.contents);
                        }
                        else {
                            println!("----------\nUser {}:\n{}\n----------", message.user_id, message.contents);
                        }
                    }
                }
                else {
                    println!("Stream write failure");
                    break;
                }
            },
            Err(e) => eprintln!("Stream write error: {}", e)
        }
    }
}