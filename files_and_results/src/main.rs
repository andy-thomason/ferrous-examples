
use std::fs::File;
use std::io::Write;
use std::io::Read;

fn open_file(name: &str) -> std::io::Result<File> {
    File::open(name)
}

fn main() -> std::io::Result<()> {
    let mut file = File::create("/tmp/1.txt")?;
    write!(file, "hello {}", 1)?;

    let mut buf = String::new();
    let mut file = open_file("/tmp/1.txt")?;
    file.read_to_string(&mut buf)?;
    println!("buf={}", buf);
    Ok(())
}
