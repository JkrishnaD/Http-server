#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection: {:?}", stream);
                handle_function(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

pub fn handle_function(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    let n = stream.read(&mut buffer).unwrap();

    let request = &buffer[..n];

    let request_str = str::from_utf8(request).unwrap();

    let request_line = request_str.lines().next().unwrap();

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();

    let status = if method == "GET" && path == "/" {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n"
    };

    println!("Request Sent: {}", request_str);

    let response = format!("{}\r\n", status);

    println!("{}", response);

    stream.write_all(&response.as_bytes()).unwrap();
}
