use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_utils::Expiration;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub token_address: Addr,
    pub recipient: String,
    pub expiration: Option<Expiration>,
    pub product: String,
    pub token_id: Option<String>,
    pub cw721_contract_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateTransaction {},
    AddTokenToContract {},
    Approve {},
    Refund {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetCountResponse)]
    ContractVaultInfo,


    #[returns(GetTransactionResponse)]
    IsTransactionOpen,
    
    #[returns(GetNftOwnerRespone)]
    GetNftOwner{
        contract_addr: String,
        token_id: String,
    }
}

#[cw_serde]
pub struct GetCountResponse {
    pub token_address: Addr,
}

#[cw_serde]
pub struct GetTransactionResponse{
    pub is_open: bool,
}

#[cw_serde]
pub struct GetNftOwnerRespone{
    pub owner_address: String,
}