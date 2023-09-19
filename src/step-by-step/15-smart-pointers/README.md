# Smart Pointers

```rust 
use std::rc::Rc;

fn main() {
    let five = Rc::new(5);
    let _a = five.clone();
    let _b = Rc::clone(&five);

    println!("There are {} strong pointers pointing to five.", Rc::strong_count(&five));
}

```
