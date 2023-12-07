fn main() {
    let ref_number: i32 = 5;
    let condition: i32 = 5;
    let is_ref_bigger: bool = ref_number > condition;
    let is_ref_smaller: bool = ref_number < condition;

    let result_text: &str;

    if is_ref_bigger {
        result_text = "bigger than";
    } else if is_ref_smaller {
        result_text = "smaller than";
    } else {
        result_text = "equals to";
    }

    println!("{ref_number} is {result_text} {condition}");
}
