use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener: Result<TcpListener, std::io::Error> = TcpListener::bind("127.0.0.1:8000");

    // We get a listener on the requested port.
    // If we have a port that we are not allowed to use, we rectify it
    // If we are trying to use a port that is already in use, we print that error and exit out.
    let listener: TcpListener = match listener {
        Ok(listener) => listener,
        Err(error) => match error.kind() {
            ErrorKind::PermissionDenied => {
                println!(
                    "WARNING: You were starting a server on a port that was below 1023. To rectify it we started the server instead on port 8000"
                );
                let l: Result<TcpListener, std::io::Error> = TcpListener::bind("127.0.0.1:8000");
                let l: TcpListener = match l {
                    Ok(l) => l,
                    Err(err) => match err.kind() {
                        ErrorKind::AddrInUse => {
                            println!(
                                "ERROR: The port requested is already in use. Stop the program using port 8000 and try again."
                            );
                            std::process::exit(0);
                        }
                        other_error => {
                            println!(
                                "ERROR: We were not able to make a connection on this port. The error faced is: {:?}",
                                other_error
                            );
                            std::process::exit(0);
                        }
                    },
                };
                l
            }
            ErrorKind::AddrInUse => panic!(
                "ERROR: The port requested is already in use. Stop the program using port 8000 and try again."
            ),
            other_error => {
                println!(
                    "ERROR: We were not able to make a connection on this port. The error faced is: {:?}",
                    other_error
                );
                std::process::exit(0);
            }
        },
    };

    for stream in listener.incoming() {
        let stream: TcpStream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                panic!(
                    "ERROR: There was an error in reading the stream of data received! The error faced is: {:?}",
                    error
                )
            }
        };

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer_reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let mut http_request: Vec<String> = Vec::new();

    // For my current application I do not process POST request so I have this commented

    // The content length above this would be a bit more unrealistic for my application
    // I think
    // let mut content_length: usize = 0;

    // `BufReader` implements the `std::io::BufRead` trait, which provides the lines method.
    // The lines method returns an iterator of `Result<String, std::io::Error>` by splitting
    // the stream of data whenever it sees a newline byte.
    // let mut lines: std::io::Lines<BufReader<&TcpStream>> = buffer_reader.lines();
    //
    // I did not use this as this was a bit limiting in the sense that I was not able
    // to read the body of the request. So I went with a different approach

    //     for line in &mut lines {
    //         // We read the each line of the request using this match function.
    //         // If we are not able to read for some reason, we print it and
    //         // gracefully exit the program.
    //         //
    //         // This is the kind of output you will see at the end of this for
    //         // loop
    //         //
    //         // GET / HTTP/1.1
    //         // Content-Type: application/json
    //         // User-Agent: PostmanRuntime/7.43.4
    //         // Accept: */*
    //         // Postman-Token: 7cd120e5-ad87-4806-8ad3-7c035e2294e7
    //         // Host: localhost:8000
    //         // Accept-Encoding: gzip, deflate, br
    //         // Connection: keep-alive
    //         // Content-Length: 46
    //         //
    //         // {
    //         //      "data" : {
    //         //          "user": "userName"
    //         //      }
    //         // }

    //         let line: String = match line {
    //             Ok(line) => line,
    //             Err(error) => {
    //                 println!(
    //                     "ERROR: We were not able to translate the line that was sent. The error faced was {:?}",
    //                     error
    //                 );
    //                 std::process::exit(0);
    //             }
    //         };

    //         if line.is_empty() {
    //             http_request.push(line.clone());
    //             break;
    //         }

    //         http_request.push(line.clone());
    //     }

    loop {
        let mut line: String = String::new();
        let bytes_read: Result<usize, std::io::Error> = buffer_reader.read_line(&mut line);

        let bytes_read = match bytes_read {
            Ok(bytes_read) => bytes_read,
            Err(error) => {
                println!("ERROR: There was some error reading the line: {:?}", error);
                std::process::exit(0);
            }
        };

        if bytes_read == 0 {
            // This means no bytes were read and we have reached the end.
            break;
        }

        let line = line.trim_end().to_string();

        // I am not processing POST request which is the reason that this block is
        // commented.
        //
        //         if line.starts_with("Content-Length: ") {
        //             let parts: Vec<&str> = line.split(':').collect();
        //             if parts.len() >= 2 {
        //                 // There should be some value present for it.
        //                 content_length = match parts[1].trim().parse() {
        //                     Ok(cl) => cl,
        //                     Err(error) => {
        //                         println!(
        //                             "WARNING: There was some error in reading the content length! The content length present is: {}. So we defaulted to 0.",
        //                             error
        //                         );
        //                         0
        //                     }
        //                 }
        //             }
        //         }

        if line.is_empty() {
            // This means we have reached the end of the headers and we exit out.

            http_request.push(line);
            break;
        } else {
            http_request.push(line);
        }
    }

    // Now here we check if it is a GET request on / endpoint
    if http_request[0] == "GET / HTTP/1.1" {
        let mut response_data: HashMap<&str, u8> = HashMap::new();
        response_data.insert("user1", 10);
        response_data.insert("user2", 20);
        response_data.insert("user3", 30);
        response_data.insert("user4", 40);
        response_data.insert("user5", 50);
        response_data.insert("user6", 60);
        response_data.insert("user7", 70);
        response_data.insert("user8", 80);
        response_data.insert("user9", 90);
        response_data.insert("user10", 100);

        let status_line: &str = "HTTP/1.1 200 OK";
        let response_data_string: String = serde_json::to_string(&response_data).unwrap();
        let response_data_length: usize = response_data_string.len();
        let response: String = format!(
            "{}\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET\r\nAccess-Control-Max-Age: 86400\r\nAccess-Control-Allow-Headers: X-PINGOTHER, Content-Type\r\nContent-Length: {}\r\n\r\n{}",
            status_line, response_data_length, response_data_string
        );
        stream
            .write_all(response.as_bytes())
            .expect("ERROR: There was some error in sending the request");
    } else {
        let method_not_allowed_response: String = get_method_not_allowed_response();
        stream
            .write_all(method_not_allowed_response.as_bytes())
            .expect("ERROR: There was some error in sending the request");
        // This is how we would read the body if we need to
        //
        //
        // Now once the headers have been added we go through the body
        //         let mut body_content: String = String::new();
        //         if content_length > 0 {
        //             let mut body: Vec<u8> = vec![0u8; content_length];

        //             body_content = if let Err(e) = buffer_reader.read_exact(&mut body) {
        //                 println!(
        //                     "ERROR: There was some error in reading the body when the content length is {}. The error is: {:?}",
        //                     content_length, e
        //                 );
        //                 String::new()
        //             } else {
        //                 String::from_utf8_lossy(&body).into_owned()
        //             }
        //         }

        //         let parsed_body: Result<_, serde_json::Error> = serde_json::from_str(&body_content);
        //         // This is not used anyway so I followed cargo's advice
        //         // let parsed_body: Value = match parsed_body {
        //         let _: Value = match parsed_body {
        //             Ok(json) => json,
        //             Err(_) => {
        //                 println!("ERROR: There was an error in parsing the body!");
        //                 Value::Null
        //             }
        //         };

        //         let mut response_data: HashMap<&str, u8> = HashMap::new();
        //         response_data.insert("user1", 10);
        //         response_data.insert("user2", 20);
        //         response_data.insert("user3", 30);
        //         response_data.insert("user4", 40);
        //         response_data.insert("user5", 50);
        //         response_data.insert("user6", 60);
        //         response_data.insert("user7", 70);
        //         response_data.insert("user8", 80);
        //         response_data.insert("user9", 90);
        //         response_data.insert("user10", 100);

        //         let status_line: &str = "HTTP/1.1 200 OK";
        //         let response_data_string: String = serde_json::to_string(&response_data).unwrap();
        //         let response_data_length: usize = response_data_string.len();
        //         let response: String = format!(
        //             "{}\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET\r\nAccess-Control-Max-Age: 86400\r\nAccess-Control-Allow-Headers: X-PINGOTHER, Content-Type\r\nContent-Length: {}\r\n\r\n{}",
        //             status_line, response_data_length, response_data_string
        //         );
        //         stream
        //             .write_all(response.as_bytes())
        //             .expect("ERROR: There was some error in sending the request");
    }
}

// This was provided by user Boiethios on stackoverflow at this link
// https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>());
// }

fn get_method_not_allowed_response() -> String {
    let mut response_data: HashMap<&str, &str> = HashMap::new();
    response_data.insert("status", "405");
    response_data.insert("error", "Method Not Allowed");
    response_data.insert(
        "message",
        "POST requests are not supported at this endpoint. Please use GET.",
    );
    response_data.insert("allowed", "[\"GET\"]");
    let response_data_string: String = serde_json::to_string(&response_data).unwrap();
    let response_data_length: usize = response_data_string.len();
    let status_line: &str = "HTTP/1.1 405 Method Not Allowed";
    format!(
        "{}\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET\r\nAccess-Control-Max-Age: 86400\r\nAccess-Control-Allow-Headers: X-PINGOTHER, Content-Type\r\nContent-Length: {}\r\n\r\n{}",
        status_line, response_data_length, response_data_string
    )
}
