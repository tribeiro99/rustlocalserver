use std::fs;
use std::io::{self, Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let file_path = if request_line.starts_with("GET / ") {
        "hello.html".to_string()
    } else {
        let path = request_line
            .split_whitespace()
            .nth(1)
            .unwrap_or("/")
            .trim_start_matches('/')
            .to_string();
        path
    };

    let (status_line, contents, content_type) = match fs::read(&file_path) {
        Ok(contents) => {
            let content_type = if file_path.ends_with(".html") {
                "text/html"
            } else if file_path.ends_with(".png") {
                "image/png"
            } else if file_path.ends_with(".css") {
                "text/css"
            } else {
                "application/octet-stream"
            };

            ("HTTP/1.1 200 OK", contents, content_type)
        }
        Err(_) => {
            let not_found = b"<h1>404 - Not Found</h1>".to_vec();
            ("HTTP/1.1 404 NOT FOUND", not_found, "text/html")
        }
    };

    let response = format!(
        "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\r\n",
        contents.len()
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
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

