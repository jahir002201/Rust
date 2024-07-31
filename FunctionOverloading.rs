trait Greet {
    fn greet(&self);
}

impl Greet for &str {
    fn greet(&self) {
        println!("Hello, {}!", self);
    }
}

impl Greet for String {
    fn greet(&self) {
        println!("Hello, {}!", self);
    }
}

fn main() {
    "Rust".greet();
    String::from("Rust").greet();
}
