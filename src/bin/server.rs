use anyhow::{Error, Result};
use lsm_engine::avl::AvlTreeMap;
use lsm_engine::binary::decode;
use lsm_engine::decoder;
use lsm_engine::executor::Executor;
use lsm_engine::memtable::Memtable;
use lsm_engine::value::Value;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{stdout, BufWriter, ErrorKind, Read, Write};
use std::mem::size_of;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::result::Result::Ok;
use std::sync::{Arc, RwLock};
use std::{fs, thread};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:33333").expect("Error. failed to bind.");
    let wal_path = Path::new("data/wal/wal.bin");
    let map = recover(wal_path).unwrap();
    let memtable: Arc<RwLock<Box<dyn Memtable<String, Value>>>> =
        Arc::new(RwLock::new(Box::new(map)));
    let wal = Arc::new(RwLock::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("data/wal/wal.bin")
            .unwrap(),
    ));
    for streams in listener.incoming() {
        match streams {
            Err(e) => {
                eprintln!("error: {}", e)
            }
            Ok(stream) => {
                let memtable = memtable.clone();
                let wal = wal.clone();
                thread::spawn(move || {
                    handler(stream, memtable, wal).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}

fn handler(
    stream: TcpStream,
    memtable: Arc<RwLock<Box<dyn Memtable<String, Value>>>>,
    wal: Arc<RwLock<File>>,
) -> Result<()> {
    println!("Connection from {}", stream.peer_addr()?);
    let mut decoder = decoder::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut executor = Executor::new(memtable, wal);
    loop {
        let decoded = decoder.decode();
        match decoded {
            Ok(c) => match executor.execute(c) {
                Ok(result) => {
                    print!("{}\n", result);
                    writer.write(format!("{}\n", result).as_bytes())?;
                    writer.flush()?;
                }
                Err(e) => {
                    print!("{}\n", e);
                    writer.write(format!("{}\n", e).as_bytes())?;
                    writer.flush()?;
                }
            },
            Err(e) => {
                print!("{}", e);
                if e.kind() == ErrorKind::UnexpectedEof {
                    return Err(Error::from(e));
                }
                writer.write(format!("{}\n", e.to_string()).as_bytes())?;
                writer.flush()?;
            }
        }
        stdout().flush()?;
    }
}

fn recover(path: &Path) -> Result<AvlTreeMap<String, Value>> {
    let mut map = AvlTreeMap::new();
    if let Ok(mut wal) = File::open(path) {
        let mut buffer = Vec::new();
        wal.read_to_end(&mut buffer)?;
        let mut index = buffer.len();
        let mut deleted_key = HashSet::new();
        while index > 0 {
            index -= size_of::<i32>();
            let binary_len =
                i32::from_le_bytes(buffer[index..(index + size_of::<i32>())].try_into()?) as usize;
            index -= binary_len;
            let (key, value) = decode(buffer[index..(index + binary_len)].to_vec())?;
            if let Some(v) = value {
                if map.search(&key).is_none() && !deleted_key.contains(&key) {
                    map.insert(key, v);
                }
            } else {
                deleted_key.insert(key);
            }
        }
    } else {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
    }
    return Ok(map);
}
