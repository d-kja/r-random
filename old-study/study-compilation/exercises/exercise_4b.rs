pub fn example() {
    // only working with positive numbers, that's the reason as to why I chose unassigned integer
    let condition: u32 = 1;

    match condition {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("other"),
    }
}
