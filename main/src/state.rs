use cosmwasm_schema::cw_serde;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Deque, Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ContractConfig {
    pub token_address: String,
    pub admin: Addr,
    pub nft_contract_address: Addr,
    pub exchange_rate: Uint128,
    pub expiration_block: u64,
}

#[cw_serde]
pub struct TransactionInfo {
    pub buyer: Addr,
    pub seller: Addr,
    pub start_block: u64,
    pub end_block: u64,
    pub product: Product,
}

#[cw_serde]
pub enum EProduct {
    NFT,
    TOKEN,
}

#[cw_serde]
pub struct Product {
    pub to_enum: EProduct,
    pub to_string: String,
    pub token_id: String,
}

impl Product {
    pub fn new(_type: &str, token_id: String) -> Product {
        let enum_type = match _type {
            "nft" => EProduct::NFT,
            "token" => EProduct::TOKEN,
            _ => panic!("Invalid product type"),
        };

        Product {
            to_enum: enum_type,
            to_string: _type.to_string(),
            token_id,
        }
    }
}

/*
= Item::new("check_transaction");
= Item::new("info");
= Item::new("config");
*/
pub const CONTRACT_CONFIG: Item<ContractConfig> = Item::new("config");
pub const TRANSACTIONS_MAP: Map<Addr, TransactionInfo> = Map::new("transactions");
pub const RESERVED_NFT: Deque<String> = Deque::new("reserved");
