#[derive(Debug)]
struct Example<T> {
    _value: T,
}

impl<T> Example<T> {
    fn new(_value: T) -> Self {
        Self { _value }
    }
}
fn main() {
    let inference_generic_example = Example::new(32);
    dbg!(inference_generic_example);

    let inference_generic_example = Example::<&str>::new("aaaa");
    dbg!(inference_generic_example);
}
