/**
 * CLI to play macro polo game
 */

use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Jufeng Zhou")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap{version = "1.0", author = "Jufeng Zhou"}]
    Marco {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Marco { name }) => {
            let answer = marco_polo::marco_polo(&name);
            println!("{}", answer);
        }
        None => {
            println!("No command given");
        }
    }
}
