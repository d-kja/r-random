struct LineItem {
    name: String,
    amount: u32,
}

impl LineItem {
    fn print_name(name: &str) {
        println!("name: {name}");
    }
    fn print_amount(&self) {
        println!("amount: {}", self.amount);
    }
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "apple".to_owned(),
            amount: 0,
        },
        LineItem {
            name: String::from("coffee"),
            amount: u32::MAX,
        },
    ];

    for item in receipt {
        LineItem::print_name(&item.name);
        LineItem::print_name(item.name.as_ref());

        item.print_amount();
    }
}
