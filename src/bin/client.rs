use std::net::TcpStream;
use std::io::{Error, BufReader, BufWriter, Write, stdout, BufRead};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:33333");
    match stream {
        Ok(s) => {
            handler(s).unwrap_or_else(|error| eprintln!("{:?}", error));
        }
        Err(e) =>  {eprintln!("error: {}", e)}
    }
}

fn handler(stream: TcpStream) -> Result<(), Error> {
    loop {
        let mut input = String::new();
        print!(">> ");
        stdout().flush()?;

        std::io::stdin().read_line(&mut input)?;

        let mut writer = BufWriter::new(&stream);
        writer.write(input.as_bytes())?;
        writer.flush()?;
        let mut reader = BufReader::new(&stream);
        let mut return_value = String::new();
        let nbytes = reader.read_line(&mut return_value)?;
        if nbytes == 0 {
            println!("receive EOF");
            break
        }
        print!("{}", return_value);
    }
    return Ok(())
}