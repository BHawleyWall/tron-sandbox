use std::process::ExitCode;

use anyhow::Result;
use clap::{ArgAction, Parser};
use tron_lib::dummy;

const SHASTA_TESTNET_FULLNODE: &str = "grpc.shasta.trongrid.io:50051";
const SHASTA_TESTNET_SOLIDITY: &str = "grpc.shasta.trongrid.io:50052";
const SHASTA_TESTNET_JSONRPC: &str = "https://api.shasta.trongrid.io/jsonrpc";

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
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

    /// Verbosity level. Pass up to four times for more verbosity.
    #[clap(short, long, action=ArgAction::Count)]
    verbose: u8,
}

fn run_cli() -> Result<()> {
    let args = Cli::parse();

    dummy(args.verbose)?;

    Ok(())
}

fn main() -> ExitCode {
    match run_cli() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
