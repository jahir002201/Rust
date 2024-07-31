extern "C" {
    fn c_function(arg: i32) -> i32;
}

fn main() {
    unsafe {
        let result = c_function(5);
        println!("Result from C: {}", result);
    }
}
