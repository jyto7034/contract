use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::Product;

#[cw_serde]
pub struct InstantiateMsg {
    pub token_address: String,
    pub nft_contract_address: String,
    pub exchange_rate: u32,
    pub expiration_block: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateTransaction {
        seller: String,
        desired_item: String,
        nft_token_id: String,
    },

    SendToken { amount: u64, address: String },
    
    SendNft { token_id: String, address: String },

    ApproveTransaction {
        buyer: String,
    },
    Refund {
        buyer: String,
    },
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
    pub product: Product,
    pub start_expiration: u64,
    pub end_expiration: u64,
}
