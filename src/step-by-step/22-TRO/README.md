# TRO

```rust 
fn factorial(n: u64) -> u64 {
    fn tail_rec(n: u64, acc: u64) -> u64 {
        if n == 0 {
            acc
        } else {
            tail_rec(n - 1, n * acc)
        }
    }
    tail_rec(n, 1)
}

fn main() {
    println!("5! = {}", factorial(5));
}

```
