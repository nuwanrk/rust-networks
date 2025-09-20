fn main() {
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
