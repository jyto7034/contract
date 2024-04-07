use cosmwasm_schema::cw_serde;
use cw_utils::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use crate::ContractError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ContractConfig {
    pub token_address: Addr,
    pub admin: Addr,
    pub nft_contract_address: Addr,
    pub exchange_rate: Uint128,
}

#[cw_serde]
pub struct TransactionInfo {
    pub buyer: Addr,
    pub seller: Addr,
    pub expiration: Expiration,
    pub product: Product,
}

#[cw_serde]
pub enum Product {
    NFT(String),
    TOKEN(String),
    NONE,
}

impl Product {
    pub fn get_nft_token(&self) -> Result<String, ContractError>{
        match &self {
            Product::NFT(token_id) => Ok(token_id.clone()),
            Product::TOKEN(token_id) => Ok(token_id.clone()),
            Product::NONE => Err(ContractError::UnauthorizedToken),
        }
    }
    
    pub fn new(product_name: String, token_id: String) -> Product {
        let product_name = product_name.to_lowercase();

        if product_name == "nft" {
            Product::NFT(token_id)
        } else {
            Product::TOKEN(token_id)
        }
    }

    pub fn none() -> Product {
        return Product::NONE;
    }
}

/*
= Item::new("check_transaction");
= Item::new("info");
= Item::new("config");
*/
pub const CONTRACT_CONFIG: Item<ContractConfig> = Item::new("config");
pub const TRANSACTIONS_MAP: Map<Addr, TransactionInfo> = Map::new("transactions");
