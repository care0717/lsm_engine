use std::io::{stdout, BufRead, BufReader, BufWriter, Error, Write};
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:33333");
    match stream {
        Ok(s) => {
            handler(s).unwrap_or_else(|error| eprintln!("{:?}", error));
        }
        Err(e) => {
            eprintln!("error: {}", e)
        }
    }
}

fn handler(stream: TcpStream) -> Result<(), Error> {
    loop {
        let mut input = String::new();
        print!(">> ");
        stdout().flush()?;

        std::io::stdin().read_line(&mut input)?;

        if input
            .split_whitespace()
            .next()
            .map_or(false, |v| v == "set")
        {
            let mut body = String::new();
            std::io::stdin().read_line(&mut body)?;
            input = input + &*body;
        }

        let mut writer = BufWriter::new(&stream);
        writer.write(input.as_bytes())?;
        writer.flush()?;
        let mut reader = BufReader::new(&stream);
        if input
            .split_whitespace()
            .next()
            .map_or(false, |v| v == "get") {
            let nbytes = reader.read_line(&mut response)?;
            if nbytes == 0 {
                println!("receive EOF");
                break;
            }
            print!("{}", response);
            continue
        }
        let mut response = String::new();
        let nbytes = reader.read_line(&mut response)?;
        if nbytes == 0 {
            println!("receive EOF");
            break;
        }
        print!("{}", response);

    }
    return Ok(());
}
