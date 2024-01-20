struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn as_tuple(self: &Self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub fn example() {
    let coords = Coords::new(32, 32);
    let (_, y) = coords.as_tuple();

    if y > 5 {
        println!("{y} is greater than 5");
        return ();
    } else if y == 5 {
        println!("{y} equals 5");
        return ();
    }

    println!("{y} is less than 5");
}
