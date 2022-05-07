use primitive_types::{H160, H256, U256};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BlockRpcResponse {
    /// the block number, 'null' when it's pending block
    #[serde(deserialize_with = "crate::decoder::deserialize_u256_from_hex_optional")]
    pub number: Option<U256>,
    /// hash of the block, 'null' when it's pending block
    #[serde(deserialize_with = "crate::decoder::deserialize_h256_optional")]
    pub hash: Option<H256>,
    /// hash of the parent block
    #[serde(rename = "parentHash")]
    pub parent_hash: H256,
    #[serde(deserialize_with = "crate::decoder::deserialize_u256_from_hex")]
    pub nonce: U256,
    /// the address of the beneficiary to whom the mining rewards were given
    pub miner: H160,
    /// integer of the difficulty for this block
    #[serde(deserialize_with = "crate::decoder::deserialize_u256_from_hex")]
    pub difficulty: U256,
    /// the maximum gas allowed in this block
    #[serde(
        rename = "gasLimit",
        deserialize_with = "crate::decoder::deserialize_u256_from_hex"
    )]
    pub gas_limit: U256,
    /// the unix timestamp for when the block was collated
    #[serde(deserialize_with = "crate::decoder::deserialize_u256_from_hex")]
    pub timestamp: U256,
    pub transactions: Vec<H256>,
    pub uncles: Vec<H256>,
}
