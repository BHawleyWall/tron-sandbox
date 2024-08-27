mod adapters;
mod entities;
mod use_cases;

use anyhow::Result;
use tracing::{debug, instrument, trace};

use crate::adapters::gateways::{
    blockchain::interface::{TronClient, TronRpc},
    telemetry::interface::init_tracing,
};

#[derive(Debug)]
pub struct Runtime {
    client: TronClient,
}

impl Runtime {
    pub fn try_new(client: TronClient) -> Result<Self> {
        const MAX_VERBOSITY: u8 = 4;

        init_tracing(MAX_VERBOSITY)?;

        trace!("Runtime initialized from injected client and using maximum verbosity.");

        Ok(Self { client })
    }

    pub async fn init(
        verbosity: u8,
        fullnode: String,
        solidity: String,
        jsonrpc: String,
    ) -> Result<Self> {
        init_tracing(verbosity)?;
        let client = TronClient::new(fullnode, solidity, jsonrpc).await;

        trace!("Runtime initialized with new client and verbosity level: {verbosity}");

        Ok(Self { client })
    }

    #[instrument]
    pub fn create_account(&self, owner_address: &str, visible: bool) -> Result<String> {
        let account = self.client.create_account(owner_address, visible)?;

        debug!("Created account: {account:?}");

        let address = String::from_utf8_lossy(&account.address).to_string();

        Ok(address)
    }
}
