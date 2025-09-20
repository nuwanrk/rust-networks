use std::{io, thread};
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::{TcpListener, TcpStream};

fn tcp_server() {
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

fn tcp_client() {
    let mut stream = TcpStream::connect("127.0.0.1:32100").expect("could not connect to the server");
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("failed to read from stdin");
        stream.write(input.as_bytes()).expect("failed to write to server");

        let mut read = BufReader::new(&stream);
        read.read_until(b'\n', &mut buffer).expect("could not read into the buffer");
        println!("{}", str::from_utf8(&buffer).expect("could not write buffer as string"));
    }
}
