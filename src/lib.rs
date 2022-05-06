use primitive_types::{H160, U256};
use serde::Deserialize;

#[derive(Debug, Copy, Clone)]
pub enum BlockIdentifier {
    Latest,
    Earliest,
    Pending,
    AtBlock(u64),
}

#[derive(Debug)]
pub enum UEthersError {

}

#[derive(Debug, Clone)]
pub struct UEthers {
    rpc: String,
}

impl UEthers {
    pub fn new(rpc: String) -> Self {
        Self {
            rpc,
        }
    }

    pub fn get_balance(&self, account: H160, at_block: BlockIdentifier) -> Result<U256, UEthersError> {

        #[derive(Debug, Deserialize)]
        struct Wrapper {
            result: String,
        }

        let wrapper: Wrapper = ureq::get(self.rpc.as_str())
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_getBalance",
                "params": [format!("{:?}", account), "latest"],
            })).unwrap().into_json().unwrap();

        let balance = U256::from_str_radix(wrapper.result.as_str(), 16).unwrap();

        Ok(balance)
    }
}


#[cfg(test)]
mod tests {
    use primitive_types::H160;
    use crate::{BlockIdentifier, UEthers};
    use std::str::FromStr;

    #[test]
    fn test_get_balance() {
        let client = UEthers::new("http://192.168.1.6:8545".to_string());
    }
}