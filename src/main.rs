pub mod one;
pub mod two;
pub mod three;
pub mod four;

use one::CommandOneArgs;
use two::CommandTwoArgs;
use three::CommandThreeArgs;
use four::CommandFourArgs;

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
}


fn main() {
    let args = Cli::parse();
    match &args.command {
      Commands::One(cmd_args) => {
         one::run(cmd_args);
      },
      Commands::Two(cmd_args) => {
         two::run(cmd_args);
      },
      Commands::Three(cmd_args) => {
         three::run(cmd_args);
      },
      Commands::Four(cmd_args) => {
         four::run(&cmd_args);
      }
   }
}
