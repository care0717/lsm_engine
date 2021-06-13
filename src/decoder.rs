use crate::command;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub struct Decoder<R: Read> {
    reader: BufReader<R>,
}

pub fn new<R: Read>(reader: R) -> Decoder<R> {
    let r = BufReader::new(reader);
    Decoder { reader: r }
}

impl<R: Read> Decoder<R> {
    pub fn decode(&mut self) -> Result<command::Command, Error> {
        let mut buf = String::new();
        let nbytes = self.reader.read_line(&mut buf)?;
        if nbytes == 0 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "got eof\n"));
        }
        let commands: Vec<&str> = buf.trim().split_whitespace().collect();

        commands
            .clone()
            .first()
            .ok_or(Error::new(ErrorKind::InvalidInput, "no content\n"))
            .and_then(move |c| match c {
                &"set" => self.decode_set(commands),
                &"get" => self.decode_get(commands),
                &"delete" => self.decode_delete(commands),
                _ => Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("unknown command: {}\n", c),
                )),
            })
    }

    fn decode_set(&mut self, commands: Vec<&str>) -> Result<command::Command, Error> {
        if commands.len() != 5 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "set command length must be 5\n",
            ));
        }
        let _key = commands[1];
        let _flag = commands[2];
        let _exptime = commands[3];
        let _bytes = commands[4]
            .parse::<usize>()
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let mut buf = String::new();
        let nbytes = self.reader.read_line(&mut buf)?;
        if nbytes == 0 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "got eof\n"));
        }
        Ok(command::new_command_set(_key.to_string(), buf))
    }

    fn decode_get(&mut self, commands: Vec<&str>) -> Result<command::Command, Error> {
        if commands.len() != 2 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "get command length must be 2\n",
            ));
        }
        let _key = commands[1];
        Ok(command::new_command_get(_key.to_string()))
    }
    fn decode_delete(&mut self, commands: Vec<&str>) -> Result<command::Command, Error> {
        if commands.len() != 2 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "delete command length must be 2\n",
            ));
        }
        let _key = commands[1];
        Ok(command::new_command_delete(_key.to_string()))
    }
}
