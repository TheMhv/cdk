use bitcoin::{Block, BlockHash, Transaction, Txid};
use corepc_types::model::{GetBlockVerboseZero, GetRawTransaction};
use jsonrpc::{simple_http::SimpleHttpTransport, Client};
use serde_json::json;
use strum_macros::Display;
use thiserror::Error;

/// Bitcoin RPC Client Errors
#[derive(Debug, Error, Display)]
pub enum Error {
    /// Cannot connect with Bitcoin JSON RPC client
    CannotConnect,
    /// Invalid URL from JSON RPC
    InvalidURL,
    /// Unable to parse params
    InvalidParams,
    /// Unable to send request
    RequestFailed,
    /// Unable to get response
    ResponseFailed,
}

/// BitcoinRPC
#[derive(Debug)]
pub struct BitcoinRPC {
    client: Client,
}

impl BitcoinRPC {
    /// Create a BitcoinRPC with Client
    pub fn new(url: &str, user: Option<String>, pass: Option<String>) -> Result<Self, Error> {
        let transport = SimpleHttpTransport::builder()
            .url(url)
            .map_err(|_| Error::InvalidURL)?
            .auth(user.unwrap_or("".into()), pass)
            .build();

        Ok(Self {
            client: Client::with_transport(transport),
        })
    }

    /// Send a JSON RPC call
    fn call_rpc<T>(
        &self,
        method: &str,
        params: Option<&serde_json::value::RawValue>,
    ) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let request = self.client.build_request(method, params);
        let response = self
            .client
            .send_request(request)
            .map_err(|_| Error::ResponseFailed)?;

        Ok(response.result().map_err(|_| Error::ResponseFailed)?)
    }

    /// Send `getblock` method to JSON RPC
    ///
    /// Return the Block
    pub fn get_block(&self, blockhash: BlockHash) -> Result<Block, Error> {
        let params = serde_json::value::to_raw_value(&json!([blockhash.to_string(), 0]))
            .map_err(|_| Error::InvalidParams)?;

        let response: GetBlockVerboseZero = self.call_rpc("getblock", Some(&params))?;

        Ok(response.0)
    }

    /// Send `getrawtransaction` method to JSON RPC
    ///
    /// Return the Transaction
    pub fn get_raw_transaction(&self, txid: &Txid) -> Result<Transaction, Error> {
        let params = serde_json::value::to_raw_value(&json!([txid.to_string(), false]))
            .map_err(|_| Error::InvalidParams)?
            .clone();

        let response: GetRawTransaction = self.call_rpc("getrawtransaction", Some(&params))?;

        Ok(response.0)
    }
}
