mod greetings {
    pub fn hello() {
        println!("Hello from the greetings module!");
    }
}

fn main() {
    greetings::hello();
}
