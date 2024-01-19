#[derive(Debug)]
struct GroceryItem {
    stock: u16,
    price: f32,
}

impl GroceryItem {
    fn print(self: &Self) {
        let formatted_str = format!(
            "Product details: \n - Stock: {}\n - Price: {}",
            self.stock, self.price
        );
        println!("{}", formatted_str);
    }
}

pub fn example() {
    let cereal = GroceryItem {
        price: 3.89,
        stock: 12,
    };

    cereal.print();
    dbg!(cereal);
}
