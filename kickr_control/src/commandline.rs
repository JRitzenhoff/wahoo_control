use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct InputArguments {
    #[command(subcommand)]
    pub run_mode: RunMode,
}

#[derive(Subcommand)]
pub enum RunMode {
    Greet,
    Add { value: u8 },
    Scan
}

/*
Doesn't work because there can't be an input argument for the Add option according to the ValueEnum trait
*/
// use clap::{Parser, ValueEnum};

// #[derive(Parser)]
// #[command(author, version, about, long_about=None)]
// pub struct InputArguments {
//     #[arg(value_enum)]
//     pub run_mode: RunMode,
// }


// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
// pub enum RunMode {
//     Greet,
//     Add,
//     Scan
// }