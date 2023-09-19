# Macro More

```rust 
macro_rules! vec_of_strings {
    ($($x:expr), *) => (vec![$($x.to_string()), *]);
}

fn main() {
    let v = vec_of_strings!("hello", "world");
    println!("{:?}", v);
}

```
