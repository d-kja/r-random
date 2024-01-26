#[derive(Debug)]
enum PromotionType {
    DEFAULT,
}

#[derive(Debug)]
enum Promotion {
    Flat(i32),
    Discount(f64),
    PromotionType(PromotionType),
}

struct Ticket {
    event: String,
    price: u16,
}

fn main() {
    let flat_value = Promotion::Flat(32);
    let discount_porcentage = Promotion::Discount(60.0);
    let promo_type = Promotion::PromotionType(PromotionType::DEFAULT);

    match flat_value {
        Promotion::Flat(value) => println!("thats the value for flat: {:?}", value),
        _ => (),
    }

    match flat_value {
        Promotion::Flat(32) => println!("the value is 32"),
        _ => (),
    }

    dbg!(promo_type, discount_porcentage, flat_value);

    let ticket = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };

    match ticket {
        Ticket { 
            price: 50, // specific param
            event // generic param 
        } => print!("price @ 50, {event}"),
        Ticket { 
            price, // generic param
            .. // ignore the rest, spread
        } => println!("price is {price}"),
    }
}
