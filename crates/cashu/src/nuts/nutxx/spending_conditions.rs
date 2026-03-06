use serde::{Deserialize, Serialize};

use crate::Conditions;

/// BTCFEE SpendingConditions
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendingConditions {
    /// Other spending conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Conditions>,
    /// Number of block confirmations required
    ///
    /// Default is 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<u64>,
}

impl SpendingConditions {
    /// Create a BTCFEE spending conditions
    pub fn new(conditions: Option<Conditions>, confirmations: Option<u64>) -> Self {
        Self {
            conditions,
            confirmations,
        }
    }
}
