use clap::{Parser, Subcommand};

/*
Used for commandline input arguments
*/
#[derive(Parser, Debug)]
pub struct SystemArguments {
    #[clap(subcommand)]
    pub run_mode: RunMode
}

#[derive(Subcommand, Debug)]
pub enum RunMode {
    Greet,
    Add { value: u8 },
    Connect
}


/*
Used for sub shell input arguments
*/

// > resistance [up|down]
// > sprint <time>
// > display_stats [--loop]

#[derive(Parser, Debug)]
#[clap(about="Specifies how the resistance should be edited")]
pub struct ResistanceArgument {
    #[clap(subcommand)]
    pub res_type: ResistanceType
}

#[derive(Subcommand, Debug)]
pub enum ResistanceType {
    Up,
    Down
}


#[derive(Parser, Debug)]
#[clap(about="Specifies how long a sprint should occur")]
pub struct SprintArgument {
    /// amount of milliseconds to sprint for
    // #[clap(short, long)]
    pub duration: u16
}

#[derive(Parser, Debug)]
#[clap(about="Specifies how the data should be displayed")]
pub struct DisplayArgument {
    pub continuous: Option<bool>
}
