use core::fmt;
use std::fmt::write;

enum Color {
    Blue,
    Yellow,
}

impl fmt::Display for Color {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Blue => write!(formatter, "Blue"),
            Color::Yellow => write!(formatter, "Yellow"),
        }
    }
}

struct Dimensions {
    width: f32,
    height: f32,
    length: f32,
}

impl fmt::Display for Dimensions {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "\n  - width: {}\n  - height: {}\n  - length: {}",
            self.width, self.height, self.length
        )
    }
}

struct Box {
    weight: f32,
    dimensions: Dimensions,
    color: Color,
}

impl fmt::Display for Box {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "weight: {}\n dimensions: {}\n color: {}",
            self.weight, self.dimensions, self.color
        )
    }
}

impl Box {
    fn new(dimensions: Dimensions, weight: f32, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        println!("Box details:\n {}", self);
    }
}

fn main() {
    let my_box = Box::new(
        Dimensions {
            width: 32.0,
            height: 32.0,
            length: 32.0,
        },
        32.0,
        Color::Yellow,
    );

    my_box.print();
}
