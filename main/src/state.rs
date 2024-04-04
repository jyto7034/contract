use cosmwasm_schema::cw_serde;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_utils::Expiration;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Permission {
    pub token_address: Addr,
    pub admin: Addr,
}

#[cw_serde]
pub struct Config {
    pub recipient: Addr,
    pub source: Addr,
    pub expiration: Option<Expiration>,
    pub exchange_rate: u128,
    pub product: Product,
    pub cw721_contract_address: Addr,
}

#[cw_serde]
pub enum Product {
    NFT(String),
    TOKEN(String),
    NONE
}

impl Product{
    pub fn new(msg: String, token_id: Option<String>) -> Product{
        if msg.to_lowercase() == "nft" && token_id.is_some(){
            return Product::NFT(msg);
        }else{
            return  Product::TOKEN(msg);
        }
    }

    pub fn none() -> Product{
        return Product::NONE
    }
}

pub const PERMISSION: Item<Permission> = Item::new("permission");
pub const CONFIG: Item<Config> = Item::new("config");
pub const TRANSACTION_STATUS: Item<bool> = Item::new("check_transaction");