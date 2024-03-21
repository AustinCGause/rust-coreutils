use clap::{
    Args,
    Parser,
    Subcommand
};

use echo::run;
mod echo;

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

    /// Currently just a test
    Ls
}

#[derive(Args)]
struct EchoArgs {
    text:Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Echo(text) => {
            run(text.text.to_owned());
        },
        Commands::Ls => { print!("Ls used"); }
    }
}
