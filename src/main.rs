use clap::{
    Parser,
    Subcommand
};

// use echo::run;

mod echo;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    #[arg(short, long)]
    echo: Vec<String>,

    #[arg(short, long)]
    name: Vec<String>,

}

#[derive(Subcommand)]
enum Commands {
}

fn main() {
    let cli = Cli::parse();

    if !cli.echo.is_empty() {
    }
}
