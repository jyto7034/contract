#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use sei_cosmwasm::{self, SeiMsg, SeiQueryWrapper};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ContractConfig, CONTRACT_CONFIG, LOCK};

const CONTRACT_NAME: &str = "crates.io:token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<SeiMsg>, ContractError> {
    let config = ContractConfig {
        admin: deps.api.addr_validate(info.sender.as_str())?,
        exchange_rate: msg.exchange_rate.parse::<u64>().unwrap(),
        denom: None,
        denom_name: None,
        one_sei: 1000000,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONTRACT_CONFIG.save(deps.storage, &config)?;
    LOCK.save(deps.storage, &false)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<SeiMsg>, ContractError> {
    match msg {
        ExecuteMsg::ChangeConfig { new_exchange_rate } => {
            execute::change_config(deps, env, info, new_exchange_rate)
        }
        ExecuteMsg::CreateDenom { denom_name } => {
            execute::create_denom(deps, env, info, denom_name)
        }
        ExecuteMsg::MintToken { amount } => {
            execute::mint_token(deps, env, info, amount)
        }
        ExecuteMsg::BurnToken { amount } => execute::burn_token(deps, env, info, amount),
        ExecuteMsg::SendToken {recipient,amount, denom } => execute::send_token(deps, env, info, recipient, amount, denom),
        ExecuteMsg::ToSei {} => execute::to_sei(deps, env, info),
        ExecuteMsg::ToToken {} => execute::to_token(deps, env, info),
        ExecuteMsg::Lock {} => execute::lock(deps, info),
    }
}

pub mod execute {
    use cosmwasm_std::{coin, coins, BankMsg, SubMsg};

    use crate::{helpers, state::LOCK};

    use super::*;

    pub fn create_denom(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        denom: String,
    ) -> Result<Response<SeiMsg>, ContractError> {
        helpers::is_admin(deps.as_ref(), env.clone(), info.sender.clone().to_string())?;

        let mut config = CONTRACT_CONFIG.load(deps.storage)?;
        config.denom_name = Some(denom.clone());
        config.denom = Some("factory/".to_string() + env.contract.address.to_string().as_ref() + "/" + denom.as_str());

        let test_create_denom = sei_cosmwasm::SeiMsg::CreateDenom {
            subdenom: config.denom_name.clone().unwrap(),
        };

        CONTRACT_CONFIG.save(deps.storage, &config)?;
        Ok(Response::new().add_message(test_create_denom))
    }

    pub fn lock(deps: DepsMut, info: MessageInfo) -> Result<Response<SeiMsg>, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        if info.sender == config.admin {
            let state = LOCK.load(deps.storage)?;
            LOCK.save(deps.storage, &!state)?;
        }
        Ok(Response::new())
    }
    
    pub fn change_config(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        new_exchange_rate: String,
    ) -> Result<Response<SeiMsg>, ContractError> {
        // admin 권한 확인
        helpers::is_admin(deps.as_ref(), env, info.sender.to_string())?;

        let mut config = CONTRACT_CONFIG.load(deps.storage)?;

        // exchange_rate 변경
        config.exchange_rate = new_exchange_rate.parse::<u64>().unwrap();

        CONTRACT_CONFIG.save(deps.storage, &config)?;
        Ok(Response::new())
    }

    pub fn mint_token(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount: String,
    ) -> Result<Response<SeiMsg>, ContractError> {
        helpers::is_admin(deps.as_ref(), env.clone(), info.sender.to_string())?;

        let config = CONTRACT_CONFIG.load(deps.storage)?;
        let amount = coin(Uint128::from(amount.parse::<u64>().unwrap()).u128(), config.denom.unwrap());

        let test_mint = sei_cosmwasm::SeiMsg::MintTokens {
            amount: amount.to_owned(),
        };
        let send_msg = SubMsg::new(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![amount],
        });

        Ok(Response::new()
            .add_message(test_mint)
            .add_submessage(send_msg))
    }

    pub fn burn_token(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount: String,
    ) -> Result<Response<SeiMsg>, ContractError> {
        helpers::is_admin(deps.as_ref(), env.clone(), info.sender.to_string())?;
        let config = CONTRACT_CONFIG.load(deps.storage)?;

        let amount = coin(Uint128::from(amount.parse::<u64>().unwrap()).u128(), config.denom.unwrap());
        let test_burn = sei_cosmwasm::SeiMsg::BurnTokens { amount };
        Ok(Response::new().add_message(test_burn))
    }
    
    pub fn send_token(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,  
        amount: String,
        denom: String,
    ) ->  Result<Response<SeiMsg>, ContractError>{
        helpers::is_admin(deps.as_ref(), env, info.sender.to_string())?;
        Ok(helpers::send_tokens(deps.api.addr_validate(recipient.as_str())?, coins(Uint128::from(amount.parse::<u64>().unwrap()).u128(), denom), "send"))
    }

    pub fn to_sei(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) ->  Result<Response<SeiMsg>, ContractError>{
        helpers::is_lock(&deps)?;
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        
        // --amount 를 통해 들어온 자금이 없을 때.
        if info.funds.is_empty() {
            return Err(ContractError::NotReceivedFunds);
        }

        // sender 가 잘못된 토큰을 보낸 경우.
        if info.funds[0].denom.clone() != config.denom.unwrap() {
            return Err(ContractError::UnauthorizedToken);
        }

        // 들어온 token 이 교환비율로 깔끔하게 나누어 떨어지지 않을 때.
        if (info.funds[0].amount.u128() % config.exchange_rate as u128) != 0{
            return Err(ContractError::BadFunds)
        }

        // 들어온 token 만큼 sei 를 사용자에게 전송
        let amount = info.funds[0].amount.u128() / config.exchange_rate as u128 * config.one_sei as u128;

        Ok(helpers::send_tokens(info.sender.clone(), coins(Uint128::from(amount).u128(), "usei"), "send"))
    }

    pub fn to_token(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) ->  Result<Response<SeiMsg>, ContractError>{
        helpers::is_lock(&deps)?;
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        
        // --amount 를 통해 들어온 자금이 없을 때.
        if info.funds.is_empty() {
            return Err(ContractError::NotReceivedFunds);
        }

        // sender 가 잘못된 토큰을 보낸 경우.
        if info.funds[0].denom.clone() != "usei" {
            return Err(ContractError::UnauthorizedToken);
        }

        // 들어온 token 이 교환비율로 깔끔하게 나누어 떨어지지 않을 때.
        if (info.funds[0].amount.u128() % config.one_sei as u128) != 0{
            return Err(ContractError::BadFunds)
        }

        // 들어온 sei 만큼 token 를 사용자에게 전송
        let amount = info.funds[0].amount.u128() / config.one_sei as u128 * config.exchange_rate as u128;

        Ok(helpers::send_tokens(info.sender.clone(), coins(Uint128::from(amount).u128(), config.denom.unwrap()), "send"))
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