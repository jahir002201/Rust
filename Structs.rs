struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Jahir"),
        age: 24,
    };
    println!("Name: {}, Age: {}", person.name, person.age);
}
