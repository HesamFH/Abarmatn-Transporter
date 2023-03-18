use super::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};
use abarmatn_mg::parser::Parser;
use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    pub fn read_file(&self, file_path: &str, values: HashMap<&str, &str>) -> Option<String> {
        // Create the full path of wanted file
        let file = format!("{}/{}", self.public_path, file_path);

        // checking the file path for possible directory traversal attack
        match fs::canonicalize(file) {
            Ok(path) => {
                // if the path was legit, just retuen the file contents :)
                // else return Option::None
                if path.starts_with(&self.public_path) {
                    let file_content = fs::read_to_string(path).ok();
                    match file_content {
                        Some(content) => Some(Parser::new(content).parse(values)),
                        None => None,
                    }
                } else {
                    println!("Directory Traversal Attack Detected!!! {}", &file_path);
                    None
                }
            }
            // Simply return None if it couldn't canonicalize the file path
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                // handle root path
                "/" => Response::new(
                    StatusCode::Ok,
                    self.read_file("index.html", HashMap::from([("name", "John Doe")])),
                ),
                // handle "hello" path
                "/hello" => {
                    Response::new(StatusCode::Ok, self.read_file("hello.html", HashMap::new()))
                }
                // handle additional file paths or 404
                path => match self.read_file(path, HashMap::new()) {
                    Some(content) => Response::new(StatusCode::Ok, Some(content)), // runs if the read_file method return something
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
