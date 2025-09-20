#![allow(unused)]

use std::{io, thread};
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::time::Duration;
use rand::{thread_rng, Rng};

const ADDRESS:&str = "127.0.0.1:32100";

pub fn udp_server() {
    println!("starting the udp server...");
    let socket = UdpSocket::bind(ADDRESS).expect("could not bind");
    loop {
        let mut buf = [0u8;1500];
        let sock = socket.try_clone().expect("failed to clone the socket");
        match sock.recv_from(&mut buf) {
            Ok((_,src)) => {
                thread::spawn(move || {
                    println!("handling a connection from {}", src);
                    sock.send_to(&buf, &src).expect("failed to send a response");
                });
            },
            Err(e) => {eprintln!("couldn't receive a datagram: {}", e)}
        }
    }
}

pub fn udp_client() {

    let socket = UdpSocket::bind(ADDRESS).expect("could not bind");
    socket.connect(ADDRESS).expect("could not connect to udp server");

    loop {
        let mut input = String::new();
        let mut buffer = [0u8; 1500];
        io::stdin().read_line(&mut input).expect("failed to read from stdin");
        socket.send(input.as_bytes()).expect("failed to write to server");

        socket.recv_from(&mut buffer).expect("could not read into buffer");
        println!("{}", str::from_utf8(&buffer).expect("could not writer buffer as string"));
    }
}


#[test]
pub fn tcp_server() {
    let listener = TcpListener::bind("127.0.0.1:32100").expect("could not bind");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || {
                    handle_tcp_conn2(s).unwrap_or_else(|e| eprintln!("{:?}", e));
                });
            }
            Err(e) => {
                eprintln!("failed: {}", e);
            }
        }
    }
}

fn handle_tcp_conn(mut stream: TcpStream) -> Result<(), Error> {
    println!("incoming connection from: {}", stream.peer_addr()?);

    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read].to_ascii_uppercase().as_slice())?;
    }
}

fn handle_tcp_conn2(mut stream: TcpStream) -> Result<(), Error> {
    println!("incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(())
        }
        let sleep = Duration::from_secs(*thread_rng().choose(&[0, 1, 2, 3, 4, 5]).unwrap());
        println!("sleeping for {:?} before replying", sleep);
        std::thread::sleep(sleep);
        stream.write(&buf[..bytes_read].to_ascii_uppercase().as_slice())?;
    }
}

pub fn tcp_client() {
    let mut stream = TcpStream::connect("127.0.0.1:32100").expect("could not connect to the server");
    client_loop(stream);
}

// with timeouts
fn tcp_client2() {
    let mut stream = TcpStream::connect("127.0.0.1:32100").expect("could not connect to the server");
    stream.set_read_timeout(Some(Duration::from_secs(3))).expect("could not set read timeout");
    client_loop(stream);
}

fn client_loop(mut stream: TcpStream) {
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
