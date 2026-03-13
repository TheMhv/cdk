//! Error types for NUT-11: Pay to Public Key (P2PK)

use thiserror::Error;

/// Nut11 Error
#[derive(Debug, Error)]
pub enum Error {
    /// Incorrect secret kind
    #[error("Secret is not a p2pk secret")]
    IncorrectSecretKind,
    /// Incorrect secret kind
    #[error("Witness is not a p2pk witness")]
    IncorrectWitnessKind,
    /// P2PK locktime has already passed
    #[error("Locktime in past")]
    LocktimeInPast,
    /// Witness signature is not valid
    #[error("Invalid signature")]
    InvalidSignature,
    /// Unknown tag in P2PK secret
    #[error("Unknown tag P2PK secret")]
    UnknownTag,
    /// Unknown Sigflag
    #[error("Unknown sigflag")]
    UnknownSigFlag,
    /// P2PK Spend conditions not meet
    #[error("P2PK spend conditions are not met")]
    SpendConditionsNotMet,
    /// Pubkey must be in data field of P2PK
    #[error("P2PK required in secret data")]
    P2PKPubkeyRequired,
    /// Unknown Kind
    #[error("Kind not found")]
    KindNotFound,
    /// Tag value not found
    #[error("Tag value not found")]
    TagValueNotFound,
    /// HTLC hash invalid
    #[error("Invalid hash")]
    InvalidHash,
    /// HTLC preimage too large
    #[error("Preimage exceeds maximum size of 32 bytes (64 hex characters)")]
    PreimageTooLarge,
    /// Witness Signatures not provided
    #[error("Witness signatures not provided")]
    SignaturesNotProvided,
    /// Duplicate signature from same pubkey
    #[error("Duplicate signature from the same pubkey detected")]
    DuplicateSignature,
    /// Impossible multisig configuration: num_sigs exceeds available pubkeys
    #[error(
        "Impossible multisig: required {required} signatures but only {available} keys available"
    )]
    ImpossibleMultisigConfiguration {
        /// Number of signatures required
        required: u64,
        /// Number of available keys
        available: u64,
    },
    /// Impossible refund multisig configuration: num_sigs_refund exceeds refund keys
    #[error("Impossible refund multisig: required {required} signatures but only {available} refund keys available")]
    ImpossibleRefundMultisigConfiguration {
        /// Number of refund signatures required
        required: u64,
        /// Number of available refund keys
        available: u64,
    },
    /// Preimage not supported in P2PK
    #[error("P2PK does not support preimage requirements")]
    PreimageNotSupportedInP2PK,
    /// SIG_ALL not supported in this context
    #[error("SIG_ALL proofs must be verified using a different method")]
    SigAllNotSupportedHere,
    /// Parse Url Error
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Parse int error
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    /// From hex error
    #[error(transparent)]
    HexError(#[from] crate::util::hex::Error),
    /// Serde Json error
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Secp256k1 error
    #[error(transparent)]
    Secp256k1(#[from] bitcoin::secp256k1::Error),
    /// NUT01 Error
    #[error(transparent)]
    NUT01(#[from] crate::nuts::nut01::Error),
    /// Secret error
    #[error(transparent)]
    Secret(#[from] crate::secret::Error),
}
