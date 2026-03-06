//! NUT-XX: Bitcoin External Fee
//!
//! <https://github.com/TheMhv/nuts/blob/feat/external-fee/xx.md>

/// BTCFEE Error module
pub mod error;
pub use error::Error;

/// BTCFEE Witness module
pub mod witness;
pub use witness::Witness;
