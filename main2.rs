use std::fs;
use std::io::{self, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1999")?;
    println!("Servidor na porta 1999...");

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

