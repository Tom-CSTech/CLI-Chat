use std::collections::HashMap;
use std::fs::OpenOptions;
#[allow(unreachable_code)]

use std::net::TcpListener;
use std::sync::Mutex;
use std::{fs::File, net::TcpStream/*, net::Shutdown*/};
use std::io::prelude::*;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::io::SeekFrom;
use std::sync::MutexGuard;
//use std::str;

use serde::{Deserialize, Serialize};
use threadpool;
use lazy_static::lazy_static;
//use crossbeam_channel::{self, Receiver};



// Statics used to track program data
lazy_static! {
    pub static ref CONNECTIONS: Mutex<Vec<&'static TcpStream>> = Mutex::new(Vec::new());
    pub static ref FILE: Mutex<File> = {
        {
            if let Err(_) = File::open("chat_history") {
                File::create("chat_history").unwrap();
            }
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true).open("chat_history").unwrap();

        Mutex::new(file)
    };
    pub static ref HISTORY: Mutex<History> = {
        let mut chat_history: Vec<u8> = Vec::new();
        {
            FILE.lock().unwrap().read_to_end(&mut chat_history).unwrap();
        }
        println!("File size: {}", chat_history.len());
        
        // Mutable runtime memory is (Hashmap) record of all chats listed (keyed) by chat ID, 
        // where a chat is a vector of messages.
        match bincode::deserialize(&chat_history) {
            Ok(data) => data,
            Err(_) => Mutex::new(HashMap::new())
        }
    };
}

/*fn set_statics() {
    let mut history = HISTORY.lock().unwrap();
    // Loading all saved chat data as bytes
    let mut chat_history: Vec<u8> = Vec::new();
    {
        FILE.lock().unwrap().read_to_end(&mut chat_history).unwrap();
    }
    println!("File size: {}", chat_history.len());
    
    // Mutable runtime memory is (Hashmap) record of all chats listed (keyed) by chat ID, 
    // where a chat is a vector of messages.
    *history = match bincode::deserialize(&chat_history) {
        Ok(data) => data,
        Err(_) => HashMap::new()
    }
}*/



#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub user_id: usize,
    pub chat_id: usize,
    pub contents: String
}

type History = HashMap<usize, Vec<Message>>;

fn main() {

    {
        let _ = HISTORY.lock().unwrap();
    }
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = threadpool::ThreadPool::new(4);

    println!("Server is listening...");
    for stream in listener.incoming() {
        
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

/// Takes the incoming HTTP request and passes it through the internal 
/// functionality from the highest level.
fn handle_connection(mut stream: TcpStream) {
    println!("Client connected!");

    //stream.shutdown(Shutdown::Both).unwrap();

    // Initialize request buffer to read request
    let mut req_buffer = [0; 1024];
    
    // Loop for sequential requests
    loop {

        // If stream is read successfully onto buffer, procede
        if let Ok(_) = stream.read(&mut req_buffer) {

            let (message, mut file, mut history) = data_prep(&req_buffer);
            let current_chat;

            println!("\n----------Message----------\nUser id: {} | Chat id: {}\nContents: {}\n", message.user_id, message.chat_id, message.contents);
 
            // Check if this is the message automatically sent when client crashes, which is empty
            if message.contents.is_empty() {
                println!("Client disconnected!");
                break;
            }

            insert_message(&mut history, &message);
            current_chat = history.get(&message.chat_id).unwrap();

            update_client(&mut stream, current_chat);
            update_file(&mut file, history);
        }
        else { // If read operation terminates without success, break loop
            break;
        }
    }
}

fn data_prep<'a>(req_buffer: &'a [u8; 1024]) -> (Message, MutexGuard<'a, File>, MutexGuard<'a, History>) {
    let message: Message = bincode::deserialize(req_buffer).unwrap();
    let mut file = FILE.lock().unwrap();
    let history = HISTORY.lock().unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

    (message, file, history)
}

// If entry for chat already exists, access and alter underlying chat to append message
// Else if chat does not yet exist, create empty vec for it, append message on new vec,
// and insert new chat in the matching hash entry
fn insert_message(history: &mut MutexGuard<History>, message: &Message) {

    match history.entry(message.chat_id) {
        Occupied(mut entry) => {
            println!("[Occupied]");
            entry.get_mut().push(message.clone());
        },
        Vacant(entry) => {
            println!("[Vacant]");
            let mut new_chat = Vec::new();
            new_chat.push(message.clone());
            entry.insert(new_chat);
        }
    };
}

// Writes updated memory of this current chat to client on connection
fn update_client(stream: &mut TcpStream, current_chat: &Vec<Message>) {

    let message_encoded = bincode::serialize(current_chat).unwrap();

    //println!("Before stream write");
    match stream.write(&message_encoded) {
        Ok(_) => stream.flush().unwrap(),
        Err(e) => eprintln!("Stream write error: {}", e)
    }
}

// Writes new memory state (all chats) to file
fn update_file(file: &mut MutexGuard<File>, history: MutexGuard<History>) {

            // &* to pass as reference to underlying type instead of MutexGuard
            let history_encoded = bincode::serialize(&*history).unwrap();

            //println!("Before file write");
            println!("Total memory: {} bytes", history_encoded.len());
            match file.write(&history_encoded) {
                Ok(_) => {/*println!("Successful write to file")*/},
                Err(e) => eprintln!("File write error: {}", e)
            }
}



pub mod test {
    #[test]
    fn data_prep() {
        
    }
}