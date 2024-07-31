#[allow(dead_code)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    fn greet(&self) {
        println!("Hello, my name is {}.", self.name);
    }
}

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message"),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
        Message::Write(text) => println!("Write message: {}", text),
    }
}

fn main() {
    let person = Person::new(String::from("Alice"), 30);
    person.greet();

    let messages = [
        Message::Quit,
        Message::ChangeColor(255, 0, 0),
        Message::Write(String::from("Hello, world!")),
    ];

    for msg in messages {
        process_message(msg);
    }
}
