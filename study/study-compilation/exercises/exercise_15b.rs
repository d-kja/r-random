// FIX BASED ON WHAT I WAS SUPPOSED TO DO.

use core::fmt;

// #[derive(Debug)]
enum Ticket {
    Backstage(f32, String),
    Vip(f32, String),
    Standard(f32),
}

impl fmt::Debug for Ticket {
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ticket::Backstage(price, holder) => write!(
                formatter,
                "Ticket: \n - type: Backstage,\n - price: {price},\n - holder: {holder}"
            ),
            Ticket::Vip(price, holder) => write!(
                formatter,
                "Ticket: \n - type: Vip,\n - price: {price},\n - holder: {holder}"
            ),
            Ticket::Standard(price) => {
                write!(formatter, "Ticket: \n - type: Standard,\n - price: {price}")
            }
        }
    }
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(32.0, "John".to_owned()),
        Ticket::Vip(23.0, "Jose".to_owned()),
        Ticket::Standard(5.0),
    ];

    for ticket in tickets {
        println!("{ticket:?}");
    }
}
