use crate::utils::setup::{FractionalNFT, NFTInfo, Nft};
use fuels::types::Identity;

pub(crate) mod fractional_nft {

    use super::*;

    pub(crate) async fn nft_info(contract: &FractionalNFT) -> Option<NFTInfo> {
        contract.methods().nft_info().call().await.unwrap().value
    }

    pub(crate) async fn supply(contract: &FractionalNFT) -> u64 {
        contract.methods().supply().call().await.unwrap().value
    }
}

pub(crate) mod nft {

    use super::*;

    pub(crate) async fn owner_of(contract: &Nft, token_id: u64) -> Option<Identity> {
        contract
            .methods()
            .owner_of(token_id)
            .call()
            .await
            .unwrap()
            .value
    }
}