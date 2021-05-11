
use std::io;
use std::io::prelude::*;

pub struct DurableFile {
    file: std::fs::File,
    needs_sync: bool,
}

impl DurableFile {
    pub fn new(file: std::fs::File) -> DurableFile {
        DurableFile {
            file: file,
            needs_sync: false
        }
    }

    pub fn close(mut self) -> io::Result<()>  {
        self.flush()
    }
}

impl std::io::Write for DurableFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.needs_sync = true;
        self.file.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.sync_all()?;
        self.needs_sync = false;
        Ok(())
    }
}

impl Drop for DurableFile {
    fn drop(&mut self) {
        if self.needs_sync {
            panic!("Tried to drop a Durablefile without flushing.");
        }
    }
}

#[test]
fn test1() -> io::Result<()> {
    let file = std::fs::File::create("/tmp/1.txt")?;
    let mut df = DurableFile::new(file);
    df.write(b"xyz")?;
    df.flush()?;
    Ok(())
}

#[test]
fn test2() -> io::Result<()> {
    let file = std::fs::File::create("/tmp/1.txt")?;
    let mut df = DurableFile::new(file);
    df.write(b"xyz")?;
    df.close()?;
    Ok(())
}
