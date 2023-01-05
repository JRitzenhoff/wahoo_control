fn main() {
    println!("Hello, world!");

    let original_value: u8 = 3;
    let summed_value: u8 = kickr_control::add_two(original_value);

    println!("{} + 2 = {}", original_value, summed_value);
}
