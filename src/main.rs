use clap::{Parser, Subcommand};

use crate::{endpoint::Endpoint, mutator::{homoglyph::mutate_homoglyphs, tlds::mutate_tlds, MutationChain, MutationChainNode}, verifier::dns::DnsVerifier};

mod endpoint;
mod mutator;
mod verifier;
mod resources;
mod fmt;

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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.debug {
        println!("Debug mode is on");
    }

    match &cli.command {
        Commands::Enumerate { fqdn } => {
            println!("Trying to enumerate phishing sites for {fqdn}");
            let endp = Endpoint::new(fqdn);

            let chain = MutationChain::new(DnsVerifier::new(), MutationChainNode::new(mutate_homoglyphs));

            chain.run(&endp).await;
        }
    }
}