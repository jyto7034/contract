use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub exchange_rate: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeConfig { new_exchange_rate: String },

    CreateDenom { denom_name: String },
    MintToken { amount: String },
    BurnToken { amount: String },
    SendToken { recipient: String, denom: String, amount: String },

    // swap token to sei
    ToSei { },

    // swap sei to token
    ToToken { },

    Lock { },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BalancesResponse)]
    GetBalances {},
}

#[cw_serde]
pub struct BalancesResponse {
    pub coin: Vec<Coin>,
}
