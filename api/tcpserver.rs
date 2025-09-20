// #![allow(dead_code)]

use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{io, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:32100").expect("could not bind");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || {
                    handle_conn(s).unwrap_or_else(|e| eprintln!("{:?}", e));
                });
            }
            Err(e) => {
                eprintln!("failed: {}", e);
            }
        }
    }
}

fn handle_conn(mut stream: TcpStream) -> Result<(), Error> {
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

