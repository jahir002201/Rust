fn main() {
    let x: i32 = 42;
    let r: *const i32 = &x;
    unsafe {
        println!("Value: {}", *r);
    }
}
