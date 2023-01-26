// Indicate that the crate can access the commandline module
pub mod bluetooth;
pub mod constants;
pub mod controller;
pub mod commandline;


pub fn say_hello() {
    println!("Hello, world!");
}

pub fn add_two(initial: u8) -> u8 {
    initial + 2
}