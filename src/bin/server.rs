use std::io::{BufWriter, Write, Error, stdout, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::thread;
use lsm_engine::decoder;

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
    let mut decoder = decoder::new(&stream);
    let mut writer = BufWriter::new(&stream);
    loop {
        let decoded = decoder.decode();
        match decoded {
            Ok(d) => {
                print!("{}", d);
                writer.write(d.as_bytes())?;
                writer.flush()?;
            }
            Err(e) => {
                print!("{}", e);
                if e.kind() == ErrorKind::UnexpectedEof {
                    return Err(e)
                }
                writer.write(format!("{}\n", e.to_string()).as_bytes())?;
                writer.flush()?;
            }
        }
        stdout().flush()?;
    }
}
