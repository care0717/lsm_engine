use lsm_engine::decoder;
use lsm_engine::executor::Executor;
use std::io::{stdout, BufWriter, Error, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, RwLock};
use lsm_engine::memtable::Memtable;
use lsm_engine::avl::AvlTreeMap;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:33333").expect("Error. failed to bind.");
    let mut memtable: Arc<RwLock<Box<dyn Memtable<String, String>>>>  = Arc::new(RwLock::new(Box::new(AvlTreeMap::new())));
    for streams in listener.incoming() {
        match streams {
            Err(e) => {
                eprintln!("error: {}", e)
            }
            Ok(stream) => {
                let memtable = memtable.clone();
                thread::spawn(move || {
                    handler(stream, memtable).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}

fn handler(stream: TcpStream, memtable: Arc<RwLock<Box<dyn Memtable<String, String>>>>) -> Result<(), Error> {
    println!("Connection from {}", stream.peer_addr()?);
    let mut decoder = decoder::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut executor = Executor::new(memtable);
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
