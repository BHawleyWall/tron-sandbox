use std::process::ExitCode;

use clap::Parser;

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
}

fn run_cli() -> Result<(), ()> {
    let args = Cli::parse();

    println!("{:?}", args);

    Ok(())
}

fn main() -> ExitCode {
    match run_cli() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
