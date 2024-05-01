use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct ContractConfig {
    pub admin: Addr,
    pub exchange_rate: u64,
    pub denom: Option<String>,
    pub denom_name: Option<String>,
    pub one_sei: u64,
}

pub const CONTRACT_CONFIG: Item<ContractConfig> = Item::new("config");
pub const LOCK: Item<bool> = Item::new("locker");