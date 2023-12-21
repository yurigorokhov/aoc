pub mod seven;
pub mod five;
pub mod four;
pub mod one;
pub mod three;
pub mod two;

use seven::CommandSevenArgs;
use five::CommandFiveArgs;
use four::CommandFourArgs;
use one::CommandOneArgs;
use three::CommandThreeArgs;
use two::CommandTwoArgs;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    One(CommandOneArgs),
    Two(CommandTwoArgs),
    Three(CommandThreeArgs),
    Four(CommandFourArgs),
    Five(CommandFiveArgs),
    Seven(CommandSevenArgs),
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::One(cmd_args) => {
            let _ = one::run(cmd_args);
        }
        Commands::Two(cmd_args) => {
            let _ = two::run(cmd_args);
        }
        Commands::Three(cmd_args) => {
            let _ = three::run(cmd_args);
        }
        Commands::Four(cmd_args) => {
            let _ = four::run(&cmd_args).unwrap();
        }
        Commands::Five(cmd_args) => {
            let _ = five::run(&cmd_args).unwrap();
        }
        Commands::Seven(cmd_args) => {
            let _ = seven::run(&cmd_args).unwrap();
        }
    }
}
