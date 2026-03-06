//! NUT-XX: Bitcoin External Fee
//!
//! <https://github.com/TheMhv/nuts/blob/feat/external-fee/xx.md>

/// BTCFEE Error module
pub mod error;
pub use error::Error;

/// BTCFEE Witness module
pub mod witness;
pub use witness::Witness;

/// BTCFEE Spending Conditions module
pub mod spending_conditions;
pub use spending_conditions::SpendingConditions;

pub mod serde_btcfee_witness;
