use crate::block_identifier::BlockIdentifier;
use crate::error::UEthersError;
use crate::models::{BlockRpcResponse, FullBlockRpcResponse, TransactionRpcResponse};
use crate::response_wrapper::{
    ResponseWrapperForBlock, ResponseWrapperForBytes, ResponseWrapperForFullBlock,
    ResponseWrapperForH256, ResponseWrapperForTransaction, ResponseWrapperForU256,
};
use primitive_types::{H160, H256, U256};
use ureq::Agent;

#[derive(Debug, Clone)]
pub struct UEthers {
    rpc: String,
    agent: Agent,
}

impl UEthers {
    /// Creates a new UEthers
    pub fn new(rpc: String) -> Self {
        Self {
            rpc,
            agent: ureq::agent(),
        }
    }

    /// Returns the balance of the account of given address
    pub fn get_balance(
        &self,
        account: H160,
        at_block: BlockIdentifier,
    ) -> Result<U256, UEthersError> {
        let wrapper: ResponseWrapperForU256 = self
            .agent
            .get(self.rpc.as_str())
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_getBalance",
                "params": [account, at_block],
            }))?
            .into_json()?;

        Ok(wrapper.result)
    }

    /// Returns the value from a storage position at a given address
    pub fn get_storage_at(
        &self,
        account: H160,
        slot: H256,
        at_block: BlockIdentifier,
    ) -> Result<H256, UEthersError> {
        let wrapper: ResponseWrapperForH256 = self
            .agent
            .get(self.rpc.as_str())
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_getStorageAt",
                "params": [account, slot, at_block],
            }))?
            .into_json()?;

        Ok(wrapper.result)
    }

    /// Returns the number of most recent block
    pub fn get_block_number(&self) -> Result<u64, UEthersError> {
        let wrapper: ResponseWrapperForU256 = self
            .agent
            .get(self.rpc.as_str())
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_blockNumber",
                "params": [],
            }))?
            .into_json()?;
        Ok(wrapper.result.as_u64())
    }

    /// Returns the number of transactions sent from an address
    pub fn get_transaction_count(
        &self,
        account: H160,
        at_block: BlockIdentifier,
    ) -> Result<U256, UEthersError> {
        let wrapper: ResponseWrapperForU256 = self
            .agent
            .get(self.rpc.as_str())
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_getTransactionCount",
                "params": [account, at_block],
            }))?
            .into_json()?;
        Ok(wrapper.result)
    }

    /// Returns code at a given address
    pub fn get_code(
        &self,
        account: H160,
        at_block: BlockIdentifier,
    ) -> Result<Vec<u8>, UEthersError> {
        let wrapper: ResponseWrapperForBytes = self
            .agent
            .get(self.rpc.as_str())
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_getCode",
                "params": [account, at_block],
            }))?
            .into_json()?;
        Ok(wrapper.result)
    }

    /// Returns information about a block by block number
    pub fn get_block_by_number(
        &self,
        at_block: BlockIdentifier,
    ) -> Result<BlockRpcResponse, UEthersError> {
        let wrapper: ResponseWrapperForBlock = self
            .agent
            .get(self.rpc.as_str())
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_getBlockByNumber",
                "params": [at_block, false],
            }))?
            .into_json()?;
        Ok(wrapper.result)
    }

    pub fn get_full_block_by_number(
        &self,
        at_block: BlockIdentifier,
    ) -> Result<FullBlockRpcResponse, UEthersError> {
        let wrapper: ResponseWrapperForFullBlock = self
            .agent
            .get(self.rpc.as_str())
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_getBlockByNumber",
                "params": [at_block, true],
            }))?
            .into_json()?;
        Ok(wrapper.result)
    }

    pub fn get_transaction_by_hash(
        &self,
        tx_hash: H256,
    ) -> Result<TransactionRpcResponse, UEthersError> {
        let wrapper: ResponseWrapperForTransaction = self
            .agent
            .get(self.rpc.as_str())
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_getTransactionByHash",
                "params": [tx_hash],
            }))?
            .into_json()?;
        Ok(wrapper.result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BlockIdentifier, UEthers};
    use primitive_types::{H160, H256};
    use std::str::FromStr;

    #[test]
    fn test_interfaces() {
        let client = UEthers::new("http://192.168.1.6:8545".to_string());

        let balance = client
            .get_balance(
                H160::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap(),
                BlockIdentifier::Latest,
            )
            .unwrap();
        assert!(!balance.is_zero());

        let code = client
            .get_code(
                H160::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap(),
                BlockIdentifier::Latest,
            )
            .unwrap();
        assert!(!code.is_empty());

        let block = client
            .get_block_by_number(BlockIdentifier::AtBlock(14727266))
            .unwrap();
        assert_eq!(
            block.hash,
            Some(
                H256::from_str(
                    "0xfb3901b3b182eb620d7531bad297a280b8bf2844b72073f107d242da33db7de9"
                )
                .unwrap()
            )
        );

        let _block = client
            .get_full_block_by_number(BlockIdentifier::AtBlock(14727266))
            .unwrap();

        let transaction = client
            .get_transaction_by_hash(
                H256::from_str(
                    "0x330909900e704a480cf4a8f00742a9356f9c901bab3b4a8099fd20c1cb8e6ebc",
                )
                .unwrap(),
            )
            .unwrap();
        dbg!(transaction);
    }
}
