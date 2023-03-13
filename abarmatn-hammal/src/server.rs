use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::{io::Read, net::TcpListener};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    /**
     * @param handler - a website route handler
     */
    pub fn run(self, mut handler: impl Handler) {
        println!("Server is running on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // Listening for a TCP request
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 2048];

                    // reading the request and writing it on buffer array
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // println!("Recived a request: {}", String::from_utf8_lossy(&buffer));

                            // Generate a Request from the buffer and create a suitable Response
                            let response: Response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                }
                                Err(err) => {
                                    handler.handle_bad_request(&err)
                                }
                            };

                            // Sending the created response and printing the error if there is any
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => {
                            println!(
                                "An Error occurred while reading data from the stream: {}",
                                e
                            )
                        }
                    }
                }
                Err(e) => println!("An Error Occurred: {}", e),
            }
        }
    }
}
