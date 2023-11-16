use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let file_path = "test.txt";
    let mut buffers = Vec::new();

    let mut file = File::open(file_path)?;
    loop {
        let bytes_read = file.read_to_end(&mut buffers)?;
        println!("bytes_read {} from file.", bytes_read);
        if bytes_read == 0 {
            break;
        }
    }

    println!("Read {} bytes from file.", buffers.len());
    Ok(())
}
