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
pub enum EProduct{
    NFT,
    TOKEN,
}

#[cw_serde]
pub struct Product{
    pub e_type: EProduct,
    pub s_type: String,
    pub _token: String,
}

impl Product{
    pub fn new(_type: &str, _token: String) -> Product{
        let e_type = match _type{
            "nft" => EProduct::NFT,
            "token" => EProduct::TOKEN,
            _ => panic!("Invalid product type"),
        };

        Product{
            e_type,
            s_type: _type.to_string(),
            _token,
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
