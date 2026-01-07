#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    collections::HashMap,
    fs,
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

    if path == "/user-agent" {
        let header = parse_headers(request_str);

        if let Some(ua) = header.get("User-Agent") {
            println!("User-Agent: {}", ua)
        }
    };

    let (status, file) = if method == "GET" && path == "/" {
        ("HTTP/1.1 200 OK", "200.html")
    } else {
        ("HTTP/1.1 404 Not Found", "404.html")
    };

    // extract the url path /echo/
    let echo_response = if let Some(echo_str) = path.strip_prefix("/echo/") {
        let len = echo_str.len();
        format!(
            "HTTP/1.1 200 OK\r\n\
            Content-Type: text/plain\r\n\
            Content-Length: {}\r\n\r\n\
            {}",
            len, echo_str
        )
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
    };

    println!("{}", file);
    println!("Request Sent: {}", request_str);

    // send the files through the stream like html
    let content = fs::read_to_string(file).unwrap();
    let response = format!(
        "{}\r\n Content-length: {}\r\n\r\n {}\r\n",
        status,
        content.len(),
        content
    );

    stream.write_all(&response.as_bytes()).unwrap();
    stream.write_all(&echo_response.as_bytes()).unwrap();
    stream.flush().unwrap()
}

fn parse_headers(request: &str) -> HashMap<&str, &str> {
    let mut header = HashMap::new();

    for line in request.lines().skip(1) {
        if line.is_empty() {
            break;
        }

        if let Some((key, value)) = line.split_once(": ") {
            header.insert(key, value);
        }
    }
    header
}
