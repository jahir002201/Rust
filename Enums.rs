#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let dir = Direction::Up;
    match dir {
        Direction::Up => println!("Going up!"),
        // The other variants are not handled here
        _ => (),
    }
}
