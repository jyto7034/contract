use cw721::Cw721QueryMsg::{Approvals, NumTokens, OwnerOf};
use cw721::{ApprovalsResponse, NumTokensResponse, OwnerOfResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    Response, StdResult, WasmMsg,
};

use crate::msg::ExecuteMsg;
use crate::state::{CONTRACT_CONFIG, LOCK};
use crate::ContractError;

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

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T, funds: Coin) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![funds],
        }
        .into())
    }
}

pub fn is_admin(deps: Deps, _env: Env, sender: String) -> Result<(), ContractError> {
    let config = CONTRACT_CONFIG.load(deps.storage)?;

    if config.admin.to_string() == sender {
        return Ok(());
    } else {
        return Err(ContractError::Unauthorized);
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

pub fn is_expired(start: u64, end: u64) -> bool {
    start >= end
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

pub fn check_approval(deps: Deps, cw721_contract_addr: Addr, token_id: String) -> StdResult<ApprovalsResponse> {
    let query_msg = Approvals {
        token_id,
        include_expired: None,
    };

    deps.querier.query(&cosmwasm_std::QueryRequest::Wasm(
        cosmwasm_std::WasmQuery::Smart {
            contract_addr: cw721_contract_addr.into_string(),
            msg: to_json_binary(&query_msg)?,
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

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CollectablesExecuteMsg {
    TransferNft(TransferNft),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransferNft {
    pub recipient: String,
    pub token_id: String,
}

impl TransferNft {
    /// serializes the message
    pub fn into_json_binary(self) -> StdResult<Binary> {
        let msg = CollectablesExecuteMsg::TransferNft(self);
        to_json_binary(&msg)
    }
    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg<T: Into<String>, C>(self, contract_addr: T) -> StdResult<CosmosMsg<C>>
    where
        C: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        let msg = self.into_json_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds: vec![],
        };
        Ok(execute.into())
    }
}

pub fn is_lock(deps: &DepsMut) -> Result<(), ContractError> {
    let state = LOCK.load(deps.storage)?;
    if state {
        Err(ContractError::Lock)
    } else {
        Ok(())
    }
}
