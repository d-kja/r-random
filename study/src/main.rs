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

fn main() {
    let flat_value = Promotion::Flat(32);
    let discount_porcentage = Promotion::Discount(60.0);
    let promo_type = Promotion::PromotionType(PromotionType::DEFAULT);

    dbg!(promo_type, discount_porcentage, flat_value);
}
