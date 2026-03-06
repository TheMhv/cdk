use bitcoin::{consensus::encode::deserialize_hex, BlockHash};
use serde::{Deserialize, Serialize};

use crate::{nutxx::Error, util::hex};

/// BTCFEE Witness
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct Witness {
    /// Blockhash
    pub blockhash: String,
    /// Preimage
    pub preimage: String,
}

impl Witness {
    /// Decode the blockhash from hex
    ///
    /// Returns the blockhash
    pub fn blockhash_data(&self) -> Result<BlockHash, Error> {
        let blockhash: BlockHash =
            deserialize_hex(&self.blockhash).map_err(|_| Error::InvalidWitnessBlockhash)?;
        Ok(blockhash)
    }

    /// Decode the preimage from hex
    ///
    /// Returns the preimage bytes data
    pub fn preimage_data(&self) -> Result<Vec<u8>, Error> {
        hex::decode(&self.preimage).map_err(|_| Error::InvalidWitnessPreimage)
    }
}
