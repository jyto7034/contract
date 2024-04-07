use cw721::Cw721ExecuteMsg::TransferNft;
use cw721::Cw721QueryMsg::{NumTokens, OwnerOf};
use cw721::{NumTokensResponse, OwnerOfResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Coin, CosmosMsg, Deps, Env, Response, StdResult, WasmMsg,
};

pub fn get_contract_address(env: Env) -> StdResult<String> {
    Ok(env.contract.address.into_string())
}

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }
}

pub fn query_balance(deps: Deps, address: Addr) -> StdResult<Vec<Coin>> {
    let balance = deps.querier.query_all_balances(address)?;
    Ok(balance)
}

pub fn send_tokens(to_address: Addr, amount: Vec<Coin>, action: &str) -> Response {
    Response::new()
        .add_message(BankMsg::Send {
            to_address: to_address.clone().into(),
            amount,
        })
        .add_attribute("action", action)
        .add_attribute("to", to_address)
}

pub fn query_owner_of(
    deps: Deps,
    cw721_contract_addr: Addr,
    token_id: String,
) -> StdResult<OwnerOfResponse> {
    let owner_of_msg = OwnerOf {
        token_id,
        include_expired: None,
    };

    deps.querier.query(&cosmwasm_std::QueryRequest::Wasm(
        cosmwasm_std::WasmQuery::Smart {
            contract_addr: cw721_contract_addr.into_string(),
            msg: to_json_binary(&owner_of_msg)?,
        },
    ))
}

pub fn query_num_of_nft(deps: Deps, cw721_contract_addr: Addr) -> StdResult<NumTokensResponse> {
    let num_of_nft_msg = NumTokens {};

    deps.querier.query(&cosmwasm_std::QueryRequest::Wasm(
        cosmwasm_std::WasmQuery::Smart {
            contract_addr: cw721_contract_addr.into_string(),
            msg: to_json_binary(&num_of_nft_msg)?,
        },
    ))
}

pub fn execute_transfer_nft(
    cw721_contract_addr: Addr,
    recipient: String,
    token_id: String,
) -> StdResult<Response> {
    let transfer_nft_msg = TransferNft {
        recipient: recipient.clone(),
        token_id: token_id.clone(),
    };

    let wasm_msg = WasmMsg::Execute {
        contract_addr: cw721_contract_addr.into_string(),
        msg: to_json_binary(&transfer_nft_msg)?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(wasm_msg))
        .add_attribute("action", "transfer_nft")
        .add_attribute("recipient", recipient)
        .add_attribute("token_id", token_id))
}
