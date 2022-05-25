use serde::de;

pub(crate) fn deserialize_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;
    let raw_hex = s.strip_prefix("0x").unwrap_or_else(|| s.as_str());
    hex::decode(raw_hex).map_err(de::Error::custom)
}
