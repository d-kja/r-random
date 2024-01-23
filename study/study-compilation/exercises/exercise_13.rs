fn main() {
    // Simple solution
    let numbers: Vec<u8> = vec![10, 20, 30, 40];

    for num in &numbers {
        let mut msg = format!("{}", num);

        if num.eq(&30) {
            msg = "thirty".to_string();
        }

        println!("value: {msg}\n");
    }

    dbg!(numbers.len());
}

fn _match_solution() {
    // Lame solution
    let numbers: Vec<u8> = vec![10, 20, 30, 40];

    for (idx, &num) in numbers.iter().enumerate() {
        let mut msg = format!("{}", num);

        match num {
            30 => msg = "thirty".to_string(),
            _ => (),
        }

        println!("iteration: {idx}\nvalue: {msg}\n\n");
    }

    dbg!(numbers.len());
}
