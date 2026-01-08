#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    net::TcpStream,
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection: {:?}", stream);
                thread::spawn(|| handle_function(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

pub fn handle_function(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    let n = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(_) => return,
    };

    let request = match str::from_utf8(&buffer[..n]) {
        Ok(request) => request,
        Err(_) => return,
    };

    let request_line = match request.lines().next() {
        Some(lines) => lines,
        None => return,
    };

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    // if endpoint is /user-agent then send the User-Agent headers to the client
    if path == "/user-agent" {
        let header = parse_headers(request);

        if let Some(ua) = header.get("User-Agent") {
            let res = format!(
                "HTTP/1.1 200 OK\r\n\
            Content-Type: text/html\r\n\
            Content-Length: {}\r\n\r\n\
            {}",
                ua.len(),
                ua
            );
            println!("User-Agent: {}", ua);
            stream.write_all(&res.as_bytes()).unwrap();
        } else {
            let res = format!("HTTP/1.1 404 NOT FOUND");
            stream.write_all(&res.as_bytes()).unwrap()
        };

        return;
    }

    // if path is "/files/directory" then send the directory file to the client from tmp
    if let Some(file) = path.strip_prefix("/files/") {
        let file_path = format!("tmp/{}", file);

        match fs::read(file_path) {
            Ok(bytes) => {
                let res = format!(
                    "HTTP/1.1 200 OK\r\n Content-Type: application/octet-stream\r\n Content-lenght:{}\r\n {}",
                    bytes.len(),
                    String::from_utf8(bytes).unwrap()
                );
                println!("{}", res);

                stream.write_all(&res.as_bytes()).unwrap();
                return;
            }
            Err(_) => {
                let res = format!("HTTP/1.1 404 NOT FOUND");
                stream.write_all(&res.as_bytes()).unwrap();
                return;
            }
        }
    }

    // extract the url path /echo/
    if let Some(echo_str) = path.strip_prefix("/echo/") {
        let len = echo_str.len();
        let res = format!(
            "HTTP/1.1 200 OK\r\n\
            Content-Type: text/plain\r\n\
            Content-Length: {}\r\n\r\n\
            {}",
            len, echo_str
        );
        stream.write_all(&res.as_bytes()).unwrap();
        return;
    }

    let (status, file) = if method == "GET" && path == "/" {
        ("HTTP/1.1 200 OK", "200.html")
    } else {
        ("HTTP/1.1 404 Not Found", "404.html")
    };

    // send the files through the stream like html
    let content = fs::read_to_string(file).unwrap();
    let response = format!(
        "{}\r\n Content-length: {}\r\n\r\n {}\r\n",
        status,
        content.len(),
        content
    );

    stream.write_all(&response.as_bytes()).unwrap();
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
