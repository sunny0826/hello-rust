fn main() {
    let value = Some(5);

    match value {
        Some(x) => println!("Got an integer: {}", x),
        None => println!("Got nothing"),
    }
}