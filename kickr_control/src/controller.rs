use std::error::Error;


/// internal library crate
use crate::commandline::{ResistanceArgument, SprintArgument, DisplayArgument};



pub fn adjust_resistance(_state: &mut (), res_arg: ResistanceArgument) -> Result<(), Box<dyn Error>> {
    println!("Adjusting the resistance to {:?}", res_arg);
    Ok(())
}

pub fn execute_sprint(_state: &mut (), sprint_arg: SprintArgument) -> Result<(), Box<dyn Error>> {
    println!("Sprinting with argument {:?}", sprint_arg);
    Ok(())
}


pub fn display_data(_state: &mut (), display_arg: DisplayArgument) -> Result<(), Box<dyn Error>> {
    println!("Displaying data according to {:?}", display_arg);
    Ok(())
}

