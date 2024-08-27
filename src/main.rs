use std::process::ExitCode;

use anyhow::{anyhow, Result};
use clap::{ArgAction, Parser, Subcommand};
use tron_lib::Runtime;

const SHASTA_TESTNET_FULLNODE: &str = "grpc.shasta.trongrid.io:50051";
const SHASTA_TESTNET_SOLIDITY: &str = "grpc.shasta.trongrid.io:50052";
const SHASTA_TESTNET_JSONRPC: &str = "https://api.shasta.trongrid.io/jsonrpc";

#[derive(Parser, Debug)]
#[command(version, about, long_about, author)]
struct Cli {
    /// Fullnode endpoint
    #[clap(short, long, default_value = SHASTA_TESTNET_FULLNODE)]
    fullnode: String,

    /// Solidity endpoint
    #[clap(short, long, default_value = SHASTA_TESTNET_SOLIDITY)]
    solidity: String,

    /// JSON-RPC endpoint
    #[clap(short, long, default_value = SHASTA_TESTNET_JSONRPC)]
    jsonrpc: String,

    /// Verbosity level. Repeat up to four times for more verbosity.
    #[clap(short, long, action=ArgAction::Count)]
    verbose: u8,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate a new account
    CreateAccount {
        /// Owner address (source of funds)
        #[clap(short, long)]
        owner_address: String,

        /// Make the account visible as a hex string or (default) base58 string
        #[clap(short, long, default_value = "true", action = ArgAction::SetFalse)]
        toggle_visible: bool,
    },
}

async fn run_cli() -> Result<()> {
    let args = Cli::parse();

    let rt = Runtime::init(args.verbose, args.fullnode, args.solidity, args.jsonrpc).await?;

    match args.command {
        Some(Commands::CreateAccount {
            owner_address,
            toggle_visible,
        }) => {
            rt.create_account(&owner_address, toggle_visible)?;
        }
        None => {
            return Err(anyhow!("No command specified."));
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    match run_cli().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e:?}");
            ExitCode::FAILURE
        }
    }
}
