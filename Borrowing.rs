fn main() {
    let mut s = String::from("hello");
    let s_ref = &s;
    println!("s_ref: {}", s_ref);
    let s_mut_ref = &mut s;
    s_mut_ref.push_str(", world!");
    println!("s_mut_ref: {}", s_mut_ref);
}
