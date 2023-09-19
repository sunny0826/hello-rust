# T

```rust 
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let max_number = largest(&numbers);
    println!("The largest number is {}", max_number);

    let chars = vec!['y', 'm', 'a', 'q'];
    let max_char = largest(&chars);
    println!("The largest char is {}", max_char);
}

```
