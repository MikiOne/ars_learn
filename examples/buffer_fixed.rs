use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let file_path = "test.txt";
    let mut buffer = [0u8; 1024];

    let mut file = File::open(file_path)?;
    let bytes_read = file.read(&mut buffer)?;

    println!("Read {} bytes from file.", bytes_read);
    println!("Read {:?} buffer from file.", &buffer);
    Ok(())
}
