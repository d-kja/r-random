enum Direction {
    Left,
    Right,
}

pub fn example() {
    let direction = Direction::Left;

    match direction {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
    }
}
