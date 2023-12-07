use core::panic;
use std::io;

struct Sizes {
    width: u32,
    height: u32,
}

fn main() {
    println!("Provide the width and height separated with a comma (,)\n");

    let mut user_input = String::new();

    let (width, height): (&str, &str) = loop {
        match io::stdin().read_line(&mut user_input) {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid input, try again");
                continue;
            }
        };

        let input_bytes = user_input.as_bytes();

        // "10,20" or "10, 20"
        let input_raw = if input_bytes.len() > 0 {
            let mut sizes: (&str, &str) = ("", "");

            // find the index containing the comma byte to obtain the position of both values
            for (index, &input_byte) in input_bytes.iter().enumerate() {
                if b',' == input_byte {
                    sizes.0 = &user_input[..index].trim_end();

                    // first case "10, 20"
                    if b' ' == input_bytes[index + 1] {
                        let idx_after_space = index + 2;
                        sizes.1 = &user_input[idx_after_space..].trim_end();

                    // second case "10 ,20"
                    } else if b' ' == input_bytes[index - 1] {
                        let idx_after_space = index + 1;
                        sizes.1 = &user_input[idx_after_space..].trim_end();

                    // third case "10,20"
                    } else {
                        let idx_after_space = index + 1;
                        sizes.1 = &user_input[idx_after_space..].trim_end();
                    }
                }
            }

            sizes
        } else {
            panic!("Invalid input! unable to convert the bytes");
        };

        break input_raw;
    };

    let Sizes { height, width } = {
        let width = width.parse::<u32>().unwrap();
        let height = height.parse::<u32>().unwrap();

        Sizes { width, height }
    };

    let rectangle_area = calc_area(Sizes { width, height });

    dbg!(rectangle_area);
}

fn calc_area(sizes: Sizes) -> u32 {
    sizes.width * sizes.height
}
