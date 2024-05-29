#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128
};
use cw2::set_contract_version;
use sei_cosmwasm::{self, DenomUnit, Metadata, SeiMsg};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ContractConfig, CONTRACT_CONFIG};

const CONTRACT_NAME: &str = "crates.io:token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response<SeiMsg>, ContractError> {
    let config = ContractConfig {
        admin: deps.api.addr_validate(info.sender.as_str())?,
    };
    
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let denom_name = "EGG";
    let denom = "factory/".to_string() + env.contract.address.as_str() + "/" + denom_name;
    let amount = 1680000000 as u128;

    let test_create_denom = sei_cosmwasm::SeiMsg::CreateDenom {
        subdenom: denom_name.to_string(),
    };

    let amount = coin(amount, denom.clone());

    let test_mint = sei_cosmwasm::SeiMsg::MintTokens {
        amount: amount.to_owned(),
    };

    let d1 = DenomUnit{
        denom: denom.clone(), 
        exponent: 0,
        aliases: vec!["uegg".to_string()],
    };
    
    let d2 = DenomUnit{
        denom: "EGG".to_string(), 
        exponent: 6,
        aliases: vec!["uegg".to_string()],
    };

    let _data = Metadata{ 
        description: "EGG is a chickegg token.".to_string(),
        denom_units: vec![d1, d2],
        base: denom,
        display: "EGG".to_string(),
        name: "EGG".to_string(),
        symbol: "EGG".to_string()
    };

    let metadata = sei_cosmwasm::SeiMsg::SetMetadata { 
        metadata: _data,
     };
    
    CONTRACT_CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
    .add_message(test_create_denom)
    .add_message(test_mint)
    .add_message(metadata))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<SeiMsg>, ContractError> {
    match msg {
        ExecuteMsg::ChangeAdmin { new_admin } => {
            execute::change_admin(deps, env, info, new_admin)
        }
        ExecuteMsg::SendToken {recipient,amount } => execute::send_token(deps, env, info, recipient, amount),
    }
}

pub mod execute {
    use cosmwasm_std::coins;

    use crate::helpers;

    use super::*;

    pub fn change_admin(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        new_admin: String,
    ) -> Result<Response<SeiMsg>, ContractError> {
        helpers::is_admin(deps.as_ref(), env.clone(), info.sender.clone().to_string())?;

        let mut config = CONTRACT_CONFIG.load(deps.storage)?;

        config.admin = deps.api.addr_validate(new_admin.as_str())?;

        CONTRACT_CONFIG.save(deps.storage, &config)?;
        Ok(Response::new())
    }
    
    pub fn send_token(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,  
        amount: String,
    ) ->  Result<Response<SeiMsg>, ContractError>{
        helpers::is_admin(deps.as_ref(), env.clone(), info.sender.to_string())?;
        let denom_name = "EGG";
        let denom = "factory/".to_string() + env.contract.address.as_str() + "/" + denom_name;
        Ok(helpers::send_tokens(deps.api.addr_validate(recipient.as_str())?, coins(Uint128::from(amount.parse::<u64>().unwrap()).u128(), denom), "send"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalances {} => to_json_binary(&query::get_balance(deps, env)),
    }
}

pub mod query {
    use cosmwasm_std::Addr;

    use crate::{
        helpers::{self, get_contract_address},
        msg::BalancesResponse,
    };

    use super::*;

    pub fn get_balance(_deps: Deps, _env: Env) -> BalancesResponse {
        let balances = helpers::query_balance(
            _deps,
            Addr::unchecked(get_contract_address(_env.clone()).unwrap()),
        )
        .unwrap();
        BalancesResponse { coin: balances }
    }
}