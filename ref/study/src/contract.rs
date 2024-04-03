#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};




#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}
/*
seid tx wasm execute sei1nkzwvlqqy5qtmgv0cw8g9z2vg78nzv9rhev7hemry9ftkawjr3jqr4yqra
'{"send_tokens":{"to":"sei17lpdjqx54em092sc5ppgg4hflkjczv5k773j4t", "amount":"10", "denom":"factory/sei18dl724gejf2l6eza9x5gg00s4nx4hkqs5dkva4/test"}}' 
--amount 10factory/sei18dl724gejf2l6eza9x5gg00s4nx4hkqs5dkva4/test
--from=wallet --node=https://sei-testnet-rpc.polkachu.com -y --broadcast-mode=block --gas=200000 --fees=20000usei --chain-id=atlantic-2
*/
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendTokens {amount, denom, to} => execute::send_tokens(deps, amount, denom, to),
    }
}

pub mod execute {
    use cosmwasm_std::{Uint128, Addr, BankMsg, Coin};

    use super::*;

    pub fn send_tokens(_deps: DepsMut, amount: Uint128, denom: String, to: Addr) -> Result<Response, ContractError> {

        Ok(Response::new().add_attribute("action", "increment")
        .add_message(BankMsg::Send { to_address: to.into_string(), amount: vec![Coin{denom, amount}] }))
    }

   
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
    }
}

pub mod query {

   
}

#[cfg(test)]
mod tests {
}