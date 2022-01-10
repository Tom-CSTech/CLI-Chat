
use std::error::Error;
pub use std::fs::File;
pub use std::io::{BufReader, Read};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub user_id: usize,
    pub chat_id: usize,
    pub contents: String
}

impl Message {
    /// Generates new response based on request data
    pub fn _new(user_id: usize, chat_id: usize, contents: String) -> Result<Message, Box<dyn Error>> {
        
        let message: Message; // Response as bytes to stream

        message = Message {
            user_id,
            chat_id,
            contents
        };

        Ok(message)
    }

    /// Converts constructed response to bytes for streaming
    pub fn _to_bytes(&mut self) -> Vec<u8> {
        let mut _data: Vec<u8> = Vec::new();

        /*data.append(&mut self.status.as_bytes().to_vec());
        for header in self.headers.clone() {
            data.append(&mut "\r\n".as_bytes().to_vec());
            data.append(&mut header.as_bytes().to_vec());
        }
        data.append(&mut "\r\n\r\n".as_bytes().to_vec());
        data.append(&mut self.body.as_bytes().to_vec());*/

        _data
    }
}