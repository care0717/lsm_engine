use std::io::{Error, Write};
use std::net::{TcpListener, TcpStream};
use std::io::BufRead;
use std::io::{BufReader, BufWriter};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:33333").expect("Error. failed to bind.");
    for streams in listener.incoming() {
        match streams {
            Err(e) => { eprintln!("error: {}", e)},
            Ok(stream) => {
                thread::spawn(move || {
                    handler(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}

fn handler(stream: TcpStream) -> Result<(), Error> {
    println!("Connection from {}", stream.peer_addr()?);

    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    loop {
        let mut buffer = String::new();
        let nbytes = reader.read_line(&mut buffer)?;
        if nbytes == 0 {
            return Ok(());
        }
        print!("{}", buffer);
        writer.write(buffer.as_bytes())?;
        writer.flush()?;
    }
}
