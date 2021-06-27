use anyhow::{Error, Result};
use lsm_engine::decoder;
use lsm_engine::executor::Executor;
use lsm_engine::memtable::{AvlMemtable, Memtable};
use lsm_engine::wal::Wal;
use std::io::{stdout, BufWriter, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::result::Result::Ok;
use std::sync::{Arc, RwLock};
use std::thread;

#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    let address = "0.0.0.0:33333";
    let listener = TcpListener::bind(address).expect("Error. failed to bind.");
    info!("Listening on {}", address);

    let wal = Wal::new(Path::new("data/wal/wal.bin")).unwrap();
    let avl_memtable = AvlMemtable::new(wal).unwrap();
    let memtable: Arc<RwLock<Box<dyn Memtable>>> = Arc::new(RwLock::new(Box::new(avl_memtable)));

    for streams in listener.incoming() {
        match streams {
            Err(e) => {
                error!("listener incoming error: {}", e)
            }
            Ok(stream) => {
                let memtable = memtable.clone();
                thread::spawn(move || {
                    handler(stream, memtable).unwrap_or_else(|error| debug!("{:?}", error));
                });
            }
        }
    }
}

fn handler(stream: TcpStream, memtable: Arc<RwLock<Box<dyn Memtable>>>) -> Result<()> {
    debug!("Connection from {}", stream.peer_addr()?);
    let mut decoder = decoder::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut executor = Executor::new(memtable);
    loop {
        let decoded = decoder.decode();
        match decoded {
            Ok(c) => match executor.execute(c) {
                Ok(result) => {
                    debug!("write response: {}", result);
                    writer.write(format!("{}\n", result).as_bytes())?;
                    writer.flush()?;
                }
                Err(e) => {
                    let error = format!("[error] {}", e);
                    debug!("write response: {}", error);
                    writer.write(format!("{}\n", error).as_bytes())?;
                    writer.flush()?;
                }
            },
            Err(e) => {
                if e.kind() == ErrorKind::UnexpectedEof {
                    return Err(Error::from(e));
                }
                let error = format!("[error] {}", e);
                debug!("write response write: {}", error);
                writer.write(format!("{}\n", error).as_bytes())?;
                writer.flush()?;
            }
        }
        stdout().flush()?;
    }
}
