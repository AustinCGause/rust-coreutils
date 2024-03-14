use clap::Parser;

mod echo;

/// GNU Coreutils Remake
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// Echo Coreutil
    #[arg(short, long)]
    echo: String,

}

fn main() {
    let args = Args::parse();

    echo::run(args.echo);
}
