use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw_utils::Expiration;

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

    // contract 자금 상태 ( Admin )
    
    // 특정 주소의 거래 상태
    #[returns(TransactionResponse)]
    GetTransactionInfo { buyer: String },
}

#[cw_serde]
pub struct TransactionResponse {
    pub desired_nft: String,
    pub seller: String,
    pub buyer: String,
    pub start_expiration: Expiration,
    pub end_expiration: Expiration,
}
