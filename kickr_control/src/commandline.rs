use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct InputArguments {
    #[arg(value_enum)]
    pub run_mode: RunMode,
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum RunMode {
    Greet,
    Add,
    Scan
}

