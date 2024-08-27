#![allow(dead_code, unused_imports, unused_variables)]

use anyhow::Result;
use k256::ecdsa::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use secrecy::SecretString;
use sha3::{Digest, Keccak256};
use tracing::{debug, error, info, instrument, trace, warn};

#[allow(clippy::enum_variant_names)]
pub mod tron_protos {
    tonic::include_proto!("protocol");
}

use tron_protos::wallet_client::WalletClient;

#[derive(Debug)]
pub struct TronClient {
    fullnode: String,
    solidity: String,
    jsonrpc: String,
    client: WalletClient<tonic::transport::Channel>,
}

impl TronClient {
    #[instrument]
    pub async fn new(fullnode: String, solidity: String, jsonrpc: String) -> Self {
        trace!("Creating new Tron client with endpoints: {fullnode}, {solidity}, {jsonrpc}");

        let client = WalletClient::connect(fullnode.to_owned())
            .await
            .expect("Failed to connect to fullnode endpoint");

        Self {
            fullnode,
            solidity,
            jsonrpc,
            client,
        }
    }

    pub fn fullnode(&self) -> &str {
        &self.fullnode
    }

    pub fn solidity(&self) -> &str {
        &self.solidity
    }

    pub fn jsonrpc(&self) -> &str {
        &self.jsonrpc
    }
}

pub trait TronRpc {
    fn create_account(&self, owner_address: &str, visible: bool) -> Result<tron_protos::Account>;
}

impl TronRpc for TronClient {
    #[instrument]
    fn create_account(&self, owner_address: &str, visible: bool) -> Result<tron_protos::Account> {
        debug!("Creating account for address: {owner_address}");

        let priv_key = SigningKey::random(&mut OsRng);
        let pub_key = VerifyingKey::from(&priv_key);

        debug!("Generated key pair: private: {priv_key:?} => public: {pub_key:?}");

        let coordinates = pub_key.to_sec1_bytes();

        debug!("Public key coordinates: {coordinates:?}");

        let mut hasher = Keccak256::new();
        hasher.update(&coordinates);

        let address = hasher.finalize();

        debug!("Address: {address:?}");

        let mut account = tron_protos::Account::default();
        account.set_type(tron_protos::AccountType::Normal);

        Ok(account)
    }
}
