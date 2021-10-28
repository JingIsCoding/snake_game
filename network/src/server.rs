use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::*;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()>  {
    let mut ostream = stream.try_clone()?;
    let mut rdr = BufReader::new(stream);
    let mut text = String::new();
    rdr.read_line(&mut text)?;
    ostream.write_all(text.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("receive...");
        handle_client(stream?);
    }
    Ok(())
}
