fn main() {
    let result = add(5, 6);
    println!("5 + 6 = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
