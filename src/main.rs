use std::fmt::format;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    //
    // ex: HTTP/1.1 200 Ok\r\n\r\n
    //

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 Ok", "index.html")
    } else {
        ("HTTP/1.1 404 Not Found", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
