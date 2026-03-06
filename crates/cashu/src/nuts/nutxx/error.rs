use thiserror::Error;

/// BTCFEE Errors
#[derive(Debug, Error)]
pub enum Error {
    // Witness Errors
    /// Invalid BTCFEE Witness
    #[error("Invalid BTCFEE witness data")]
    InvalidWitness,
    /// Invalid BTCFEE Witness Blockhash
    #[error("Invalid BTCFEE witness blockhash data")]
    InvalidWitnessBlockhash,
    /// Invalid BTCFEE Witness Preimage
    #[error("Invalid BTCFEE witness preimage data")]
    InvalidWitnessPreimage,
}
