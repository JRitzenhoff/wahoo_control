use clap::{Parser as _};
use kickr_control::commandline::{InputArguments, RunMode};

fn main() {
   let cli = InputArguments::parse();

   match cli.run_mode {
    RunMode::Greet => {
        kickr_control::say_hello();
    }
    RunMode::Add { value } => {
        // let original_value: u8 = 7;
        let summed_value: u8 = kickr_control::add_two(value);
    
        println!("{} + 2 = {}", value, summed_value); 
    }
    RunMode::Scan => {
        println!("Scanning has not been implemented yet");
    }
   }
}
