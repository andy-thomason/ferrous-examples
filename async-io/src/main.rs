use async_std::io;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;

fn tcp_server() -> io::Result<()> {
    async_std::task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            eprintln!("new stream");
            let (reader, writer) = &mut (&stream, &stream);
            io::copy(reader, writer).await?;
        }
        Ok(())
    })
}

fn tcp_client() -> io::Result<()> {
    async_std::task::block_on(async {
        let mut stream = TcpStream::connect("www.google.com:80").await?;
        stream
            .write_all(b"GET /search?q=rust HTTP/1.1\nHost: www.google.com\n\n")
            .await?;

        let mut buf = vec![0u8; 1024];
        let n = stream.read(&mut buf).await?;
        println!("{}", String::from_utf8_lossy(&mut buf));
        Ok(())
    })
}

fn main() {
    if true {
        tcp_client().unwrap();
    } else {
        tcp_server().unwrap();
    }
}
