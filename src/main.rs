pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;

use one::CommandOneArgs;
use two::CommandTwoArgs;
use three::CommandThreeArgs;
use four::CommandFourArgs;
use five::CommandFiveArgs;

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
}


fn main() {
    let args = Cli::parse();
    match &args.command {
      Commands::One(cmd_args) => {
         let _ = one::run(cmd_args);
      },
      Commands::Two(cmd_args) => {
         let _ = two::run(cmd_args);
      },
      Commands::Three(cmd_args) => {
         let _ = three::run(cmd_args);
      },
      Commands::Four(cmd_args) => {
         let _ = four::run(&cmd_args).unwrap();
      },
      Commands::Five(cmd_args) => {
         let _ = five::run(&cmd_args).unwrap();
      }
   }
}
