pub mod one;

use one::CommandOneArgs;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
   #[command(subcommand)]
   command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
   One(CommandOneArgs)
}


fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::One(cmd_args) => one::run(cmd_args),
     }
}
