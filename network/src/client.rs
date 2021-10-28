use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut content = String::new();
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let msg = "what the".to_string();
    stream.write_all(msg.as_bytes())?;
    stream.read_to_string(&mut content)?;
    println!("server says.. {}", content);
    Ok(())
}
