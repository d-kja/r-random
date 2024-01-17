pub fn example() {
    let mut loop_count: i32 = 1;

    loop {
        let stop_condition = &loop_count > &4;

        if stop_condition {
            break;
        }

        println!("Current loop: {loop_count}");
        loop_count += 1;
    }
}
