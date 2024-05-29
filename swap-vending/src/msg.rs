use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub token_address: String,
    pub nft_contract_address: String,
    pub exchange_rate: u32,
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeConfig { token_address: String, nft_contract_address: String, exchange_rate: u32 },

    ChangeAdmin { new_admin: String },

    CreateTransaction {
        desired_item: String,
        nft_token_id: String,
    },

    SendToken {
        denom: Option<String>,
        amount: u64,
        address: String,
    },

    SendNft {
        token_id: String,
        address: String,
    },

    Lock {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BalancesResponse)]
    GetBalances {
    },

    #[returns(ConfigResponse)]
    GetConfig {
    },
}

#[cw_serde]
pub struct BalancesResponse {
    pub coin: Vec<Coin>
}

#[cw_serde]
pub struct ConfigResponse {
    pub token_address: String,
    pub nft_contract_address: String,
    pub exchange_rate: u32,
}
