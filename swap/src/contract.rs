#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::executer::executer;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ContractConfig, CONTRACT_CONFIG, LOCK};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = ContractConfig {
        token_address: msg.token_address,
        admin: deps.api.addr_validate(info.sender.as_str())?,
        nft_contract_address: deps.api.addr_validate(msg.nft_contract_address.as_str())?,
        exchange_rate: Uint128::from(msg.exchange_rate),
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONTRACT_CONFIG.save(deps.storage, &config)?;
    LOCK.save(deps.storage, &false)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateTransaction {
            desired_item,
            nft_token_id,
        } => execute::create_transaction(_deps, _env, _info, desired_item, nft_token_id),

        ExecuteMsg::SendNft { token_id, address } => {
            execute::send_nft(_deps.as_ref(), _info, _env, token_id, address)
        }

        ExecuteMsg::SendToken {amount,address, denom } => {
            execute::send_token(_deps.as_ref(), _info, _env, amount, address, denom)
        }
        ExecuteMsg::Lock {} => execute::lock(_deps, _info),
    }
}

pub mod execute {
    use self::executer::transfer_nft;
    use crate::helpers;
    use crate::state::{EProduct, Product, LOCK};
    use cosmwasm_std::{coins, Addr};

    use super::*;

    pub fn lock(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        if info.sender == config.admin {
            let state = LOCK.load(deps.storage)?;
            LOCK.save(deps.storage, &!state)?;
        }
        Ok(Response::new())
    }

    pub fn send_token(
        deps: Deps,
        info: MessageInfo,
        _env: Env,
        amount: u64,
        address: String,
        denom: Option<String>
    ) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        // admin 권한 확인
        if info.sender.clone() == config.admin {
             let denom_temp = if let Some(_denom) = denom{
                _denom.clone()
            }else{
                config.token_address.clone()
            };
            Ok(helpers::send_tokens(
                Addr::unchecked(address),
                coins(amount as u128, denom_temp),
                "send",
            ))
        } else {
            Err(ContractError::Unauthorized)
        }
    }

    pub fn send_nft(
        deps: Deps,
        info: MessageInfo,
        _env: Env,
        token_id: String,
        address: String,
    ) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        // admin 권한 확인
        if info.sender.clone() == config.admin {
            transfer_nft(deps, address, token_id)
        } else {
            Err(ContractError::Unauthorized)
        }
    }

    // Porduct 구매 Transaction 생성
    pub fn create_transaction(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        desired_item: String,
        token_id: String,
    ) -> Result<Response, ContractError> {
        helpers::is_lock(&deps)?;
        let config = CONTRACT_CONFIG.load(deps.storage)?;

        let product = Product::new(desired_item.as_str(), token_id.clone());

        match &product.to_enum {
            EProduct::NFT => {
                executer::create_transaction_token_to_nft(deps, env, info, product.token_id.clone())
            }
            EProduct::TOKEN => executer::create_transaction_nft_to_token(
                deps.as_ref(),
                info,
                env,
                &config,
                product.token_id.clone(),
            ),
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalances {} => to_json_binary(&query::get_balances(deps, _env)),
    }
}

pub mod query {
    use cosmwasm_std::Addr;

    use crate::{helpers::{self, get_contract_address}, msg::BalancesResponse};

    use super::*;

    pub fn get_balances(
        _deps: Deps,
        _env: Env,
    ) -> BalancesResponse {
        let balances = helpers::query_balance(_deps, Addr::unchecked(get_contract_address(_env.clone()).unwrap())).unwrap();
        BalancesResponse{
            coin: balances,
        }
    }
}
