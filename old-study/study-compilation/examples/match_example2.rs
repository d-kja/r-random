struct Type {
    A: i32,
    B: bool,
}

enum SecondType {
    A,
    B,
}

pub fn example() {
    let condition = true;

    match condition {
        true => {}
        false => {}
    };

    let condition = 5;

    match condition {
        5 => {}
        _ => {}
    };

    let condition = Type { A: 32, B: false };

    // In this case it's kind of stupid because both items exists in the struct
    match condition {
        B => {
            println!("b");
        }
        A => {
            println!("a");
        }
    }

    // But you can do the same with Enums
    let condition = SecondType::A;

    // In this case it's kind of stupid because both items exists in the struct
    match condition {
        SecondType::A => {
            println!("A");
        }
        SecondType::B => {
            println!("B");
        }
    }
}
