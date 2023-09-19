# Error Chain

```rust 
use std::fs::File;
use std::io;
use std::io::Read;

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file_contents("hello.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => println!("Failed to read the file: {:?}", error),
    }
}

```
