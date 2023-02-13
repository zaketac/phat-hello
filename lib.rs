#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

// pink_extension is short for Phala ink! extension
use pink_extension as pink;

#[pink::contract(env=PinkEnvironment)]
mod phat_hello2 {
    use super::pink;
    use ink_prelude::{format, string::String};
    use pink::{http_get, PinkEnvironment};
    use scale::{Decode, Encode};
    use serde::Deserialize;
    // you have to use crates with `no_std` support in contract.
    use serde_json_core;

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InvalidEthAddress,
        HttpRequestFailed,
        InvalidResponseBody,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// Defines the storage of your contract.
    /// All the fields will be encrypted and stored on-chain.
    /// In this stateless example, we just add a useless field for demo.
    #[ink(storage)]
    pub struct PhatHello {
        demo_field: bool,
    }

    #[derive(Deserialize, Encode, Clone, Debug, PartialEq)]
    pub struct EtherscanResponse<'a> {
        jsonrpc: &'a str,
        id: &'a str,
        result: &'a str,
    }

    impl PhatHello {
        /// Constructor to initializes your contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { demo_field: true }
        }

        /// A function to handle direct off-chain Query from users.
        /// Such functions use the immutable reference `&self`
        /// so WILL NOT change the contract state.
        #[ink(message)]
        pub fn get_eth_balance(&self, block: String) -> Result<String> {
            if !block.starts_with("0") && block.len() != 42 {
                return Err(Error::InvalidEthAddress);
            }

            // get info block with HTTP requests to Etherscan
            // you can send any HTTP requests in Query handler
            let resp = http_get!(format!(
                "http://rpc-juno.mib.tech/block?height={}",
                block
            ));
            if resp.status_code != 200 {
                return Err(Error::HttpRequestFailed);
            }

            let result: EtherscanResponse = serde_json_core::from_slice(&resp.body)
                .or(Err(Error::InvalidResponseBody))?
                .0;
            Ok(String::from(result.result))
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test a simple use case of our contract.
        
        #[ink::test]
        fn it_works() {
            // when your contract is really deployed, the Phala Worker will do the HTTP requests
            // mock is needed for local test
            pink_extension_runtime::mock_ext::mock_all_ext();

            let phat_hello = PhatHello::new();
            let block = String::from("0xD0fE316B9f01A3b5fd6790F88C2D53739F80B464");
            let res = phat_hello.get_eth_balance(block.clone());
            assert!(res.is_ok());

            // run with `cargo +nightly test -- --nocapture` to see the following output
            println!("Block {} gets {} Wei", block, res.unwrap());
        }
    }
}
