use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeAdmin { new_admin: String },

    SendToken { recipient: String, amount: String },

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
