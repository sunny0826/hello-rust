# Error Handling

Rust 使用 Result 类型来处理可能失败的操作。

```rust 
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Failed to open the file: {:?}", error),
    }
}

```
