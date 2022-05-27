use primitive_types::{H160, H256, U256};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BlockRpcResponseTemplate<TxType> {
    /// the block number, 'null' when it's pending block
    pub number: Option<U256>,
    /// hash of the block, 'null' when it's pending block
    pub hash: Option<H256>,
    /// hash of the parent block
    #[serde(rename = "parentHash")]
    pub parent_hash: H256,
    pub nonce: U256,
    /// the address of the beneficiary to whom the mining rewards were given
    pub miner: H160,
    /// integer of the difficulty for this block
    pub difficulty: U256,
    /// the maximum gas allowed in this block
    #[serde(rename = "gasLimit")]
    pub gas_limit: U256,
    /// the unix timestamp for when the block was collated
    pub timestamp: U256,
    pub transactions: Vec<TxType>,
    pub uncles: Vec<H256>,
}
pub type BlockRpcResponse = BlockRpcResponseTemplate<H256>;
pub type FullBlockRpcResponse = BlockRpcResponseTemplate<TransactionRpcResponse>;

#[derive(Debug, Clone, Deserialize)]
pub struct TransactionRpcResponse {
    /// hash of the block where this transaction was in, 'null' when it's pending
    #[serde(rename = "blockHash")]
    pub block_hash: Option<H256>,
    /// block number where this transaction was in, 'null' when it's pending
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U256>,
    /// address of the sender
    pub from: H160,
    pub gas: U256,
    /// gas provided by the sender
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<U256>,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: Option<U256>,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: Option<U256>,
    /// hash of the transaction
    pub hash: H256,
    /// the data sent along with the transaction
    #[serde(deserialize_with = "crate::decoder::deserialize_bytes")]
    pub input: Vec<u8>,
    pub nonce: U256,
    /// address of the receiver, 'null' when it's a contract creation transaction
    pub to: Option<H160>,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<U256>,
    /// value transferred in Wei
    pub value: U256,
    /// ECDSA recovery id
    pub v: U256,
    /// ECDSA signature r
    pub r: U256,
    /// ECDSA signature s
    pub s: U256,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransactionReceipt {
    #[serde(rename = "transactionHash")]
    pub transaction_hash: H256,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: U256,
    #[serde(rename = "blockHash")]
    pub block_hash: H256,
    #[serde(rename = "blockNumber")]
    pub block_number: U256,
    pub from: H160,
    pub to: H160,
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: U256,
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<H160>,
    // TODO: add logs and logsBloom
    pub status: U256,
}

#[cfg(test)]
mod tests {
    use crate::TransactionRpcResponse;

    #[test]
    fn test_deserialize() {
        let s = r#"{"blockHash":"0x92a1165fb8003bf1b6e8a2cf28e805cbbd2c700aa199f96d404061368202900c","blockNumber":"0xe129ae","from":"0xfdf1c31b8b5617f457e31bca1ea53d5a612ec640","gas":"0x12eda1","gasPrice":"0x239c6a1cc6","maxPriorityFeePerGas":"0x0","maxFeePerGas":null,"hash":"0x317bd6d667eccb4b543c2c41596a97099649f06888481aa6b2f3b8e9f15027aa","input":"0x2e91811c000000000000000000000000000000000000000000000000000000000000011c00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000001030000000000000000000000951b928f2a8ba7ce14cc418cfebfee30a57294c30000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000017336eac4000000000000000000000000000000000000000000000000321cfbb6bc715dbb","nonce":"0xb5","to":"0xb84bd1221667164a4b3a4b798ec79b5874d1ff71","transactionIndex":"0xc","value":"0x0","type":"0x2","accessList":[],"chainId":"0x1","v":"0x0","r":"0x51c10ee160738546d566cba58cb2d16649000e8acba4dd3e20ddbe43985f4575","s":"0xa357028b521e051b688aef43d03ec68a04b0298223402641c7feffe75a7b9f4"}"#;
        assert!(serde_json::from_str::<TransactionRpcResponse>(s).is_ok());
    }

    #[test]
    fn test_deserialize_optional() {
        // add some nulls
        let s = r#"{"blockHash":null,"blockNumber":null,"from":"0xfdf1c31b8b5617f457e31bca1ea53d5a612ec640","gas":"0x12eda1","gasPrice":"0x239c6a1cc6","maxPriorityFeePerGas":"0x0","maxFeePerGas":null,"hash":"0x317bd6d667eccb4b543c2c41596a97099649f06888481aa6b2f3b8e9f15027aa","input":"0x2e91811c000000000000000000000000000000000000000000000000000000000000011c00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000001030000000000000000000000951b928f2a8ba7ce14cc418cfebfee30a57294c30000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000017336eac4000000000000000000000000000000000000000000000000321cfbb6bc715dbb","nonce":"0xb5","to":"0xb84bd1221667164a4b3a4b798ec79b5874d1ff71","transactionIndex":"0xc","value":"0x0","type":"0x2","accessList":[],"chainId":"0x1","v":"0x0","r":"0x51c10ee160738546d566cba58cb2d16649000e8acba4dd3e20ddbe43985f4575","s":"0xa357028b521e051b688aef43d03ec68a04b0298223402641c7feffe75a7b9f4"}"#;
        assert!(serde_json::from_str::<TransactionRpcResponse>(s).is_ok());
    }
}
