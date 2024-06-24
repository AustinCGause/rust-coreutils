use std::fs::File;

use clap::{
    Args,
    Parser,
    Subcommand
};

mod echo;
use echo::echo;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    #[command(subcommand)]
    command: Commands,   

}

#[derive(Subcommand)]
enum Commands {
    /// Echo text
    Echo(EchoArgs),

    Cat(CatArgs),
    /// Currently just a test
    Ls
}

#[derive(Args)]
struct EchoArgs {
    tokens:Vec<String>,
}

#[derive(Args)]
struct CatArgs {
    file2:File,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Echo(tokens) => {
            echo(tokens.tokens.to_owned());
        },
        Commands::Cat(file) => {
            print!("{}",file.file2);   
        },
        Commands::Ls => { print!("Ls used"); }
    }
}
