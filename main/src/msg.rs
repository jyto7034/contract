use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub token_address: String,
    pub nft_contract_address: String,
    pub exchange_rate: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateTransaction {
        seller: String,
        desired_item: String,
        nft_token_id: String,
    },

    ApproveTransaction {
        buyer: String,
        product: String,
        nft_token_id: String,
    },
    AddTokenToContract {},
    Refund {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetCountResponse)]
    ContractVaultInfo,

    #[returns(GetTransactionResponse)]
    IsTransactionOpen { buyer: String },

    #[returns(GetNftOwnerRespone)]
    GetNftOwner {
        contract_addr: String,
        token_id: String,
    },
}

#[cw_serde]
pub struct GetCountResponse {
    pub token_address: Addr,
}

#[cw_serde]
pub struct GetTransactionResponse {
    pub is_open: bool,
}

#[cw_serde]
pub struct GetNftOwnerRespone {
    pub owner_address: String,
}
