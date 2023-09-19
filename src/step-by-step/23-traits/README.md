# traits

```rust 
trait Speak {
    fn speak(&self);
}

struct Human;

impl Speak for Human {
    fn speak(&self) {
        println!("Hello!");
    }
}

fn main() {
    let person = Human;
    person.speak();
}


```
