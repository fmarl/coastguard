use clap::{Parser, Subcommand};

use crate::endpoint::Endpoint;

mod endpoint;
mod mutator;
mod verifier;
mod resources;

#[derive(Parser, Debug)]
#[command(name = "coastguard")]
#[command(about = "", long_about = "Coastguard tries to enumerate phishing sites based on a website")]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[arg(short, long)]
    levenshtein_distance: Option<u32>,

    /// Subcommands
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Enumerate {
        fqdn: String,
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        println!("Debug mode is on");
    }

    match &cli.command {
        Commands::Enumerate { fqdn } => {
            println!("Trying to enumerate phishing sites for {fqdn}");
            let endp = Endpoint::new(fqdn);
        }
    }
}