#[derive(Debug)]
struct Box {
    height: u32,
    width: u32,
}

impl Box {
    fn is_squared(&self) -> bool {
        self.height == self.width
    }

    fn matches_box(self: &Self, another_box: &Box) -> bool {
        let is_same_width = self.width == another_box.width;
        let is_same_height = self.height == another_box.height;

        is_same_width && is_same_height
    }

    fn create_square(size: u32) -> Box {
        Self {
            height: size,
            width: size,
        }
    }

    fn new(width: u32, height: u32) -> Self {
        Box { height, width }
    }
}

pub fn example() {
    let box_example = Box {
        width: 32,
        height: 32,
    };

    let second_box = Box::create_square(32);

    dbg!(&box_example, &second_box);

    println!("is the box a square? {}", box_example.is_squared());
    println!(
        "does it match the other box? {}",
        box_example.matches_box(&second_box)
    );

    dbg!(Box::new(43, 23));
}
