use clap::{
    Args,
    Parser,
    Subcommand
};

mod echo;
use echo::run;

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
    tokens:Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Echo(tokens) => {
            run(tokens.tokens.to_owned());
        },
        Commands::Ls => { print!("Ls used"); }
    }
}
