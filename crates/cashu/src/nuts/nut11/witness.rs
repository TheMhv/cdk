//! Witness struct for NUT-11: Pay to Public Key (P2PK)

use serde::{Deserialize, Serialize};

/// P2PK Witness
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct P2PKWitness {
    /// Signatures
    pub signatures: Vec<String>,
}

impl P2PKWitness {
    #[inline]
    /// Check id Witness is empty
    pub fn is_empty(&self) -> bool {
        self.signatures.is_empty()
    }
}
