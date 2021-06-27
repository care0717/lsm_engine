use anyhow::{Error, Result};
use lsm_engine::decoder;
use lsm_engine::executor::Executor;
use lsm_engine::memtable::{AvlMemtable, Memtable};
use lsm_engine::sstable::{HashMapSSTable, SSTable};
use lsm_engine::wal::Wal;
use std::io::{stdout, BufWriter, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::result::Result::Ok;
use std::sync::{Arc, RwLock};
use std::{fs, thread};

#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    let address = "0.0.0.0:33333";
    let listener = TcpListener::bind(address).expect("Error. failed to bind.");
    info!("Listening on {}", address);
    let wal_path = Path::new("data/wal/wal.bin");
    if let Some(parent) = wal_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).unwrap();
        }
    }
    let wal = Wal::new(wal_path).unwrap();
    let avl_memtable = AvlMemtable::new(wal).unwrap();
    let memtable: Arc<RwLock<Box<dyn Memtable>>> = Arc::new(RwLock::new(Box::new(avl_memtable)));

    let sstable_dir = Path::new("data/sstable");
    if !sstable_dir.exists() {
        fs::create_dir_all(sstable_dir).unwrap();
    }
    let hashmap_sstable = HashMapSSTable::new(sstable_dir).unwrap();
    let sstable: Arc<RwLock<Box<dyn SSTable>>> = Arc::new(RwLock::new(Box::new(hashmap_sstable)));

    for streams in listener.incoming() {
        match streams {
            Err(e) => {
                error!("listener incoming error: {}", e)
            }
            Ok(stream) => {
                let memtable = memtable.clone();
                let sstable = sstable.clone();
                thread::spawn(move || {
                    handler(stream, memtable, sstable)
                        .unwrap_or_else(|error| debug!("{:?}", error));
                });
            }
        }
    }
}

fn handler(
    stream: TcpStream,
    memtable: Arc<RwLock<Box<dyn Memtable>>>,
    sstable: Arc<RwLock<Box<dyn SSTable>>>,
) -> Result<()> {
    debug!("Connection from {}", stream.peer_addr()?);
    let mut decoder = decoder::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut executor = Executor::new(memtable, sstable);
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
