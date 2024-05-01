use cosmwasm_std::{Addr, BankMsg, Coin, Deps, DepsMut, Env, Response, StdResult, Uint128};
use sei_cosmwasm::SeiMsg;

use crate::{state::{CONTRACT_CONFIG, LOCK}, ContractError};

pub fn query_balance(deps: Deps, address: Addr) -> StdResult<Vec<Coin>> {
    let balance = deps.querier.query_all_balances(address)?;
    Ok(balance)
}

pub fn get_contract_address(env: Env) -> StdResult<String> {
    Ok(env.contract.address.into_string())
}

pub fn send_tokens(to_address: Addr, amount: Vec<Coin>, action: &str) -> Response<SeiMsg> {
    Response::new()
        .add_message(BankMsg::Send {
            to_address: to_address.clone().into(),
            amount,
        })
        .add_attribute("action", action)
        .add_attribute("to", to_address)
}

pub fn is_admin(deps: Deps, _env: Env, sender: String) -> Result<(), ContractError> {
    let config = CONTRACT_CONFIG.load(deps.storage)?;

    if config.admin.to_string() == sender {
        return Ok(());
    } else {
        return Err(ContractError::Unauthorized);
    }
}

pub fn convert_uint128_to_u64(value: Uint128) -> Result<u64, String> {
    if value.u128() > u64::MAX as u128 {
        Err("Value exceeds u64 max limit".to_string())
    } else {
        Ok(value.u128() as u64)
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
