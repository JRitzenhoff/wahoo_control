use clap::{Parser as _}; // need this import so that the InputArguments Trait is acknowledged
use shellfish::{Shell, clap_command};

use kickr_control::constants::{SUBSHELL_PREFACE, RESISTANCE_ARG_KEY, SPRINT_ARG_KEY, DISPLAY_ARG_KEY};
use kickr_control::commandline::{SystemArguments, RunMode, ResistanceArgument, SprintArgument, DisplayArgument};
use kickr_control::controller::{adjust_resistance, execute_sprint, display_data};


fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    launch_run_mode(arguments);
}

fn launch_run_mode(parse_arguments: Vec<String>) {
    let cli = SystemArguments::parse_from(parse_arguments);

    match cli.run_mode {
        RunMode::Greet => {
            kickr_control::say_hello();
        }
        RunMode::Add { value } => {
            // let original_value: u8 = 7;
            let summed_value: u8 = kickr_control::add_two(value);
        
            println!("{} + 2 = {}", value, summed_value); 
        }
        RunMode::Connect => {
            launch().unwrap();
            // kickr_control::controller::launch(io::stdin, io::stdout);
        }
    }
}


fn launch() -> Result<(), Box<dyn std::error::Error>> {
    // Define a shell
    let mut shell = Shell::new((), SUBSHELL_PREFACE);


    shell.commands.insert(RESISTANCE_ARG_KEY, clap_command!((), ResistanceArgument, adjust_resistance));
    shell.commands.insert(SPRINT_ARG_KEY, clap_command!((), SprintArgument, execute_sprint));
    shell.commands.insert(DISPLAY_ARG_KEY, clap_command!(() , DisplayArgument, display_data));

    shell.run().unwrap();

    Ok(())
}
