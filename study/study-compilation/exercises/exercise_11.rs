struct GroceryItem {
    id: u32,
    quantity: u16,
}

impl GroceryItem {
    fn get_quantity(item: &GroceryItem) {
        println!("The available amount is {}", item.quantity);
    }

    fn get_id(item: &GroceryItem) {
        println!("The item id is {}", item.id);
    }
}
fn main() {
    let grocery_item: GroceryItem = GroceryItem {
        id: 1,
        quantity: 43,
    };

    GroceryItem::get_quantity(&grocery_item);
    GroceryItem::get_id(&grocery_item);
}
