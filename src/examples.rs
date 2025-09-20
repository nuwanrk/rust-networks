use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("incoming connection from: {}", stream.peer_addr()?);

    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
    }
}

#[allow(dead_code)]
pub fn test_handle_client() {
    let listener = TcpListener::bind("127.0.0.1:32100").expect("cloud not bind");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || {
                    handle_client(s).unwrap_or_else(|e| eprintln!("{:?}", e));
                });
            }
            Err(e) => {
                eprintln!("failed: {}", e);
            }
        }
    }
}
