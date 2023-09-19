# Thread

```rust 
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for _ in 1..10 {
            println!("Hi from the spawned thread!");
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    for _ in 1..5 {
        println!("Hi from the main thread!");
        thread::sleep(std::time::Duration::from_millis(1));
    }

    handle.join().unwrap();
}

```
