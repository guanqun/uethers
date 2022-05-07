use crate::models::BlockRpcResponse;
use primitive_types::{H256, U256};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ResponseWrapperForU256 {
    #[serde(deserialize_with = "crate::decoder::deserialize_u256_from_hex")]
    pub result: U256,
}

#[derive(Deserialize)]
pub(crate) struct ResponseWrapperForH256 {
    pub result: H256,
}

#[derive(Deserialize)]
pub(crate) struct ResponseWrapperForBytes {
    #[serde(deserialize_with = "crate::decoder::deserialize_bytes")]
    pub result: Vec<u8>,
}

#[derive(Deserialize)]
pub(crate) struct ResponseWrapperForBlock {
    pub result: BlockRpcResponse,
}