use serde::{Serialize, Serializer};

#[derive(Debug, Copy, Clone)]
pub enum BlockIdentifier {
    Latest,
    Earliest,
    Pending,
    AtBlock(u64),
}

impl ToString for BlockIdentifier {
    fn to_string(&self) -> String {
        match *self {
            BlockIdentifier::Latest => "latest".to_string(),
            BlockIdentifier::Earliest => "earliest".to_string(),
            BlockIdentifier::Pending => "pending".to_string(),
            BlockIdentifier::AtBlock(block) => format!("0x{:x}", block),
        }
    }
}

impl Serialize for BlockIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            BlockIdentifier::Latest => serializer.serialize_str("latest"),
            BlockIdentifier::Earliest => serializer.serialize_str("earliest"),
            BlockIdentifier::Pending => serializer.serialize_str("pending"),
            BlockIdentifier::AtBlock(block) => {
                serializer.serialize_str(format!("0x{:x}", block).as_str())
            }
        }
    }
}

impl From<u64> for BlockIdentifier {
    fn from(block: u64) -> Self {
        BlockIdentifier::AtBlock(block)
    }
}
