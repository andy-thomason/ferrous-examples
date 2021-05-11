use std::net::{TcpListener, TcpStream};
use std::io;

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    // ...
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
