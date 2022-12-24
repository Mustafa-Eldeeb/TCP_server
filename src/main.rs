use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

//handles a single client
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let stream_add = stream.peer_addr()?;
    println!("Incoming conn from {}", stream_add);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&mut buf[..bytes_read]);
        println!("Incoming conn from {:?}", buf);
    }
}
fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("Error: {}", e)
            }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
