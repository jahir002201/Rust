fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // This will cause a compile error
    // println!("{}", s1); 
    println!("{}", s2);
}
