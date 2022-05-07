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

#[derive(Debug, Clone, Deserialize)]
pub struct TransactionRpcResponse {
    /// hash of the block where this transaction was in, 'null' when it's pending
    #[serde(
        rename = "blockHash",
        deserialize_with = "crate::decoder::deserialize_h256_optional"
    )]
    pub block_hash: Option<H256>,
    /// block number where this transaction was in, 'null' when it's pending
    #[serde(
        rename = "blockNumber",
        deserialize_with = "crate::decoder::deserialize_u256_from_hex_optional"
    )]
    pub block_number: Option<U256>,
    /// address of the sender
    pub from: H160,
    /// gas provided by the sender
    #[serde(
        rename = "gasPrice",
        deserialize_with = "crate::decoder::deserialize_u256_from_hex_optional"
    )]
    pub gas_price: Option<U256>,
    #[serde(
        rename = "maxPriorityFeePerGas",
        deserialize_with = "crate::decoder::deserialize_u256_from_hex_optional"
    )]
    pub max_priority_fee_per_gas: Option<U256>,
    #[serde(
        rename = "maxFeePerGas",
        deserialize_with = "crate::decoder::deserialize_u256_from_hex_optional"
    )]
    pub max_fee_per_gas: Option<U256>,
    /// hash of the transaction
    pub hash: H256,
    /// the data sent along with the transaction
    #[serde(deserialize_with = "crate::decoder::deserialize_bytes")]
    pub input: Vec<u8>,
    #[serde(deserialize_with = "crate::decoder::deserialize_u256_from_hex")]
    pub nonce: U256,
    /// address of the receiver, 'null' when it's a contract creation transaction
    pub to: Option<H160>,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<U256>,
    /// value transferred in Wei
    #[serde(deserialize_with = "crate::decoder::deserialize_u256_from_hex")]
    pub value: U256,
    /// ECDSA recovery id
    #[serde(deserialize_with = "crate::decoder::deserialize_u256_from_hex")]
    pub v: U256,
    /// ECDSA signature r
    pub r: H256,
    /// ECDSA signature s
    pub s: H256,
}
