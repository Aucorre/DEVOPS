use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
fn main() {
    println!("Listening on 0.0.0.0:8000/ping");
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut headers_map:HashMap<String, String> = HashMap::new();
    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    for header in request.iter() {
        let parts: Vec<&str> = header.splitn(2, ':').collect();
        if parts.len() == 2 {
            headers_map.insert(parts[0].to_string(), parts[1].to_string());
        } else {
            headers_map.insert("Method".to_string(), parts[0].to_string());
        }
    }
    let response = if request[0].eq_ignore_ascii_case("GET /ping HTTP/1.1") {
        format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
            serde_json::to_string(&headers_map).unwrap()
        )
    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string()
    };
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}