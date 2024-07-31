mod greetings {
    pub fn hello() {
        println!("Hello from the module!");
    }
}

fn main() {
    greetings::hello();
}
