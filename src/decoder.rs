use primitive_types::{H256, U256};
use serde::de;
use std::str::FromStr;
use ureq::serde_json::Value;

pub(crate) fn deserialize_u256_from_hex<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;
    U256::from_str_radix(s.as_str(), 16).map_err(de::Error::custom)
}

pub(crate) fn deserialize_u256_from_hex_optional<'de, D>(
    deserializer: D,
) -> Result<Option<U256>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: Value = de::Deserialize::deserialize(deserializer)?;
    if s.is_null() {
        Ok(None)
    } else {
        U256::from_str_radix(
            s.as_str()
                .ok_or_else(|| de::Error::custom("must be a string encoded u256"))?,
            16,
        )
        .map(|x| Some(x))
        .map_err(de::Error::custom)
    }
}

pub(crate) fn deserialize_h256_optional<'de, D>(deserializer: D) -> Result<Option<H256>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: Value = de::Deserialize::deserialize(deserializer)?;
    if s.is_null() {
        Ok(None)
    } else {
        H256::from_str(
            s.as_str()
                .ok_or_else(|| de::Error::custom("must be a string encoded h256"))?,
        )
        .map(|hash| Some(hash))
        .map_err(de::Error::custom)
    }
}

pub(crate) fn deserialize_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;
    let raw_hex = s.strip_prefix("0x").unwrap_or_else(|| s.as_str());
    hex::decode(raw_hex).map_err(de::Error::custom)
}
