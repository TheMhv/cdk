//! Serde helpers for BTCFEE Witness

use crate::nutxx::Witness;
use serde::{de, ser, Deserialize, Deserializer, Serializer};

/// Serialize [BTCFEEWitness] as stringified JSON
pub fn serialize<S>(x: &Witness, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&serde_json::to_string(&x).map_err(ser::Error::custom)?)
}

/// Deserialize [BTCFEEWitness] from stringified JSON
pub fn deserialize<'de, D>(deserializer: D) -> Result<Witness, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(de::Error::custom)
}
