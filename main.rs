use std::fs;
use std::io::{self, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = &http_request[0];
    let (method, path) = parse_request_line(request_line);

    if method == "GET" {
        let file_path = if path == "/" {
            "hello.html" 
        } else {
            &path[1..]  
        };

        let response = match fs::read_to_string(file_path) {
            Ok(contents) => {
                let status_line = "HTTP/1.1 200 OK";
                let length = contents.len();
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
            }
            Err(_) => {
                let status_line = "HTTP/1.1 404 NOT FOUND";
                let contents = "<h1>404 - Not Found</h1>";
                let length = contents.len();
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
            }
        };

        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn parse_request_line(request_line: &str) -> (&str, &str) {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() >= 2 {
        (parts[0], parts[1]) 
    } else {
        ("", "/") 
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1999")?;
    println!("Servidor escutando na porta 1999...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream); 
            }
            Err(e) => {
                eprintln!("Erro ao aceitar conexão: {}", e);
            }
        }
    }
    Ok(())
}

