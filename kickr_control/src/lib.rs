// Indicate that the crate can access the commandline module
pub mod commandline;
pub mod bluetooth;




pub fn say_hello() {
    println!("Hello, world!");
}

pub fn add_two(initial: u8) -> u8 {
    initial + 2
}