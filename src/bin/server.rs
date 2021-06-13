use lsm_engine::decoder;
use lsm_engine::executor::Executor;
use std::io::{stdout, BufWriter, Error, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:33333").expect("Error. failed to bind.");
    for streams in listener.incoming() {
        match streams {
            Err(e) => {
                eprintln!("error: {}", e)
            }
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
    let mut executor = Executor::new_avl();
    loop {
        let decoded = decoder.decode();
        match decoded {
            Ok(c) => {
                let result = executor.execute(c);
                print!("{}\n", result);
                writer.write(format!("{}\n", result).as_bytes())?;
                writer.flush()?;
            }
            Err(e) => {
                print!("{}", e);
                if e.kind() == ErrorKind::UnexpectedEof {
                    return Err(e);
                }
                writer.write(format!("{}\n", e.to_string()).as_bytes())?;
                writer.flush()?;
            }
        }
        stdout().flush()?;
    }
}
