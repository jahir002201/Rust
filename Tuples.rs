fn main() {
    let tup: (i32, f64, bool) = (500, 6.4, true);
    let (x, y, z) = tup; // Destructuring the tuple
    
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}
