use crate::command::Command;
use crate::value::Value;
use std::io;
use std::io::BufRead;

pub struct Decoder<R: io::Read> {
    reader: io::BufReader<R>,
}

pub fn new<R: io::Read>(reader: R) -> Decoder<R> {
    let r = io::BufReader::new(reader);
    Decoder { reader: r }
}

impl<R: io::Read> Decoder<R> {
    pub fn decode(&mut self) -> Result<Command, io::Error> {
        let mut buf = String::new();
        let nbytes = self.reader.read_line(&mut buf)?;
        if nbytes == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "got eof"));
        }
        let commands: Vec<&str> = buf.trim().split_whitespace().collect();

        commands
            .clone()
            .first()
            .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "no content"))
            .and_then(move |c| match c {
                &"set" => self.decode_set(commands),
                &"get" => self.decode_get(commands),
                &"delete" => self.decode_delete(commands),
                &"stats" => Ok(Command::new_stats()),
                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("unknown command: {}", c),
                )),
            })
    }

    fn decode_set(&mut self, commands: Vec<&str>) -> Result<Command, io::Error> {
        if commands.len() != 5 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "set command length must be 5",
            ));
        }
        let key = commands[1];
        let flags = commands[2]
            .parse::<usize>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let exptime = commands[3]
            .parse::<usize>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let _bytes = commands[4]
            .parse::<usize>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let mut buf = String::new();
        let nbytes = self.reader.read_line(&mut buf)?;
        if nbytes == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "got eof"));
        }
        let value = Value::new(buf.trim().parse().unwrap(), flags, exptime);
        Ok(Command::new_set(key.to_string(), value))
    }

    fn decode_get(&self, commands: Vec<&str>) -> Result<Command, io::Error> {
        if commands.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "get command length must be 2",
            ));
        }
        let key = commands[1];
        Ok(Command::new_get(key.to_string()))
    }
    fn decode_delete(&self, commands: Vec<&str>) -> Result<Command, io::Error> {
        if commands.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "delete command length must be 2",
            ));
        }
        let key = commands[1];
        Ok(Command::new_delete(key.to_string()))
    }
}
