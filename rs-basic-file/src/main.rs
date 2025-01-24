use std::fs::*;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;


fn basic_file_handling() -> std::io::Result<()> {

    let path_location = r"./toto.txt";
    let path = Path::new(path_location);

    // Create a file in a simple way

    let mut file = File::create(path)?;

    file.write_all(b"Hello, world!")?;
    file.write_all("\nHello, world!".as_bytes())?;

    // Create an append file
    
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(b"\nHello, world!\n")?;

    let some_vec = vec![1, 2, 3, 4, 5];
    let mut some_str = String::new();
    for a in some_vec {
        some_str.push_str(&a.to_string());
    }
    file.write_all(some_str.as_bytes())?;


    // read file lines

    let file = File::open(path)?;
    let file_buffer = BufReader::new(file);

    for line in file_buffer.lines() {
        println!("{:?}", line?);
    }

    Ok(())
}

fn main() {
    basic_file_handling().unwrap();
}
