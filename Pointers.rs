fn main() {
    let x = 5;
    let y: *const i32 = &x;
    unsafe {
        println!("y points to: {}", *y);
    }
}
