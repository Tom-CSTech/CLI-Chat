#![allow(non_snake_case)]

pub use std::str;
pub use std::io::prelude::*;

use crate::response::*;
use crate::endpoints::{self, *};

/// Takes a raw HTTP request and returns a Response struct
/// the includes status and the requested data itself (the body)
/// on success. A request is validated here, before any API 
/// calls: if a Response is not returned, the request was invalid.
pub fn fulfil_req(request: &[u8]) -> Result<Response, Box<dyn Error>> {

    // Get only first line (the rest is not needed)
    let request = str::from_utf8(request).unwrap().lines().next().unwrap().as_bytes();

    if request.ends_with(b" HTTP/1.1") {
        let request = &request[..(request.len()-9)];
        
        if request.starts_with(b"GET /api") {
            let request = &request[8..];

            if request.starts_with(b"/ping") {
                let ping = endpoints::ping()?;
                Ok(Response::new(
                    ping.0,
                    ping.1
                ).unwrap())
            }

            else if request.starts_with(b"/posts?") {
                let request = &request[7..];

                // Pass on response containing posts, or error message.
                match endpoints::get_posts(&request) {
                    Ok((status, body)) => {
                        Ok(Response::new(
                            status,
                            body
                        ).unwrap())
                    }
                    Err(body) => {
                        Ok(Response::new(
                            "HTTP/1.1 400 BAD REQUEST".to_string(),
                            body.to_string()
                        ).unwrap())
                    }
                }
            }
            else {
                Err("Request Invalid".into())
            }

        }
        else {
            Err("Request Invalid".into())
        }

    }
    else {
        Err("Request Invalid".into())
    }
}