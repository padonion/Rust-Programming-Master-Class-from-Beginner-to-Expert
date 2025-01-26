use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //handle_connection(stream);
        handle_connection2(stream);
    }
}

fn _handle_connection(mut stream: TcpStream) {
    let buf_reader = std::io::BufReader::new(&mut stream);
    let http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();

    println!("Request: {:#?}", http_request);

    /*
    Response syntax:
    HTTP-version status-code reason-phrase CRLF
    headers CRLF
    message-body

    example: HTTP/1.1 200 OK\r\n\r\n
    */

    let status_line = "HTTP/1.1 200 OK\r\n";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    let response =
        format!("{} Content-Length: {}\r\n\r\n{}", status_line, length, contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn handle_connection2(mut stream: TcpStream) {

    let buf_reader = std::io::BufReader::new(&mut stream);
    let mut request_line = buf_reader.lines().next();
    let (status_line, filename) = match request_line.unwrap().unwrap().as_str() {
        "GET / HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("index.html")),
        "GET /hello HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("hello.html")),
        _ => (Some("HTTP/1.1 404 NOT FOUND\r\n"), Some("404.html")),
    };

    let contents = fs::read_to_string(filename.unwrap()).unwrap();
    let response = format!("{} Content-Length: {}\r\n\r\n{}", status_line.unwrap(), contents.len(), contents);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}