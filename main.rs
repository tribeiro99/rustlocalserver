use std::net::{SocketAddrV4, Ipv4Addr};
use syscalls::{syscall, Sysno}; 

fn main() {
    let mut socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    socket.set_port(1999);

    let message = b"Aqui jaze o  syscall, bem vindo!!!\n";

    unsafe {
        let stdout: usize = 1; 
        match syscall!(Sysno::write, stdout, message.as_ptr(), message.len()) {
            Ok(bytes_written) => {
                println!("Bytes escritos: {}", bytes_written);
                println!("{}", std::str::from_utf8(message).unwrap());
            }
            Err(err) => {
                eprintln!("Erro ao escrever: {:?}", err);
            }
        }
    }

    assert_eq!(socket.port(), 1999);
}

