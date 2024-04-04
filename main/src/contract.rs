
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::helpers::get_contract_address;
use crate::msg::{QueryMsg, ExecuteMsg, InstantiateMsg};
use crate::state::{Config, Permission, Product, CONFIG, PERMISSION, TRANSACTION_STATUS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:main";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    let permission = Permission {
        token_address: Addr::unchecked(msg.token_address.as_str()),
        admin: info.sender.clone(),
    };
    
    let config = Config {
        recipient: deps.api.addr_validate(&msg.recipient)?,
        source: info.sender,
        expiration: msg.expiration,
        exchange_rate: 800000,
        product: Product::new(msg.product, msg.token_id),
        cw721_contract_address: deps.api.addr_validate(&msg.cw721_contract_address)?,
    };
    
    println!("Init! {}, {}", permission.admin, permission.token_address);
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    if let Some(expiration) = msg.expiration {
        if expiration.is_expired(&_env.block) {
            return Err(ContractError::Expired { expiration });
        }
    }

    TRANSACTION_STATUS.save(deps.storage, &false)?;
    PERMISSION.save(deps.storage, &permission)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddTokenToContract{} => execute::add_token_to_contract(_deps,_env, info),
        // CreateTransaction 를 실행하면서 --amount 로 자금이 들어와야함.
        ExecuteMsg::CreateTransaction {} => execute::create_transaction(_deps,_env, info),
        ExecuteMsg::Approve{}=>todo!(),
        ExecuteMsg::Refund{}=>todo!(),
    }
}

pub mod execute {
    use cosmwasm_std::{coin, BankMsg, Coin, Uint128};

    use crate::helpers;

    use super::*;

    fn create_transaction_nft_to_token(deps: Deps, _info: MessageInfo, _env: Env, _permission: &Permission, config: Config, income_token: String) -> Result<Response, ContractError>{
        // sender 가 잘못된 nft 를 보내는 경우.
        // 존재하지 않는 nft 을 보내는 경우 Wasm runtime Failed 뜸.
        let res = helpers::query_owner_of(deps, config.cw721_contract_address.clone(), income_token.clone())?;
        if res.owner == helpers::get_contract_address(_env)?{
            return Err(ContractError::DoesNotOwnNFT);
        }

        // contract 의 token 이 적당한가?
        let balances = helpers::query_balance(deps, config.cw721_contract_address.clone())?; 
        if balances.is_empty() {
            return Err(ContractError::BalanceQueryFailed)
        }
        let wallet = balances.iter().find(|wallet| { wallet.denom == _permission.token_address });
        if let Some(wallet) = wallet{
            if wallet.amount < Uint128::new(800000){
                return Err(ContractError::NotEnoughContractTokens)
            }
        }
        
        // 토큰 전송
        helpers::send_tokens(config.recipient, vec![coin(config.exchange_rate, _permission.token_address.clone())], "send_token");

        
        Ok(Response::new().add_attribute("create_transaction_nft_to_token", "Created"))
    }
    

    // token 으로 nft 를 사는 경우
    fn create_transaction_token_to_nft(deps: Deps, info: MessageInfo, permission: &Permission, config: &Config, wanted_token: String) -> Result<Response, ContractError>{
        // sender 가 잘못된 토큰을 보낸 경우.
        if info.funds[0].denom.clone() != permission.token_address
        {
            return Err(ContractError::UnauthorizedToken);
        }
        
        // NFT 가 남아있는가?
        let res = helpers::query_num_of_nft(deps, config.cw721_contract_address.clone())?;
        if res.count <= 0{
            return Err(ContractError::NotEnoughContractNFT)
        }
        
        // NFT 전송
        helpers::execute_transfer_nft(config.cw721_contract_address.clone(), config.recipient.clone().into_string(), wanted_token.clone())?;
        
        Ok(Response::new().add_attribute("create_transaction_token_to_nft", "Created"))
    }
    pub fn create_transaction(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError>{
        let config = CONFIG.load(deps.storage)?;
        let permission = PERMISSION.load(deps.storage)?;
        
        // --amount 를 통해 들어온 자금이 없을 때.
        if info.funds.is_empty() {
            return Err(ContractError::NotReceivedFunds);
        }

        // funds 의 자금이 교환 비율과 맞지 않을 때.
        // TODO!!
        //     funds 가 수수료가 떼진 상태서 오는가?
        if info.funds[0].amount.u128() != config.exchange_rate {
            return Err(ContractError::NotMatchExchangeRate);
        }

        TRANSACTION_STATUS.save(deps.storage, &true)?;
        match &config.product {
            Product::NFT(token) => create_transaction_nft_to_token(deps.as_ref(), info, _env, &permission, config.clone(), token.clone()),
            Product::TOKEN(token) => create_transaction_token_to_nft(deps.as_ref(), info, &permission, &config, token.clone()),
            Product::NONE => todo!(),
        }
    }
    
    pub fn execute_approve(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let config = CONFIG.load(deps.storage)?;
        // if info.sender != config.arbiter {
        //     return Err(ContractError::Unauthorized {});
        // }
    
        // throws error if the contract is expired
        if let Some(expiration) = config.expiration {
            if expiration.is_expired(&env.block) {
                return Err(ContractError::Expired { expiration });
            }
        }

        let permission = PERMISSION.load(deps.storage)?;
    
        Ok(crate::helpers::send_tokens(config.recipient, vec![coin(800000, permission.token_address)], "approve"))
    }


    // Contract 에 token 충전
    pub fn add_token_to_contract(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        // --amount 를 통해 들어온 자금이 없을 때.
        if info.funds.is_empty() {
            return Err(ContractError::NotReceivedFunds);
        }

        // funds 의 자금이 0 일 때.
        if info.funds[0].amount.u128() == 0 {
            return Err(ContractError::NoFunds);
        }

        let permission = PERMISSION.load(deps.storage)?;

        // sender 의 주소가 admin 권한이 아닌 경우 혹은 잘못된 토큰을 보내고 있는 경우.
        if info.sender.into_string() != permission.admin
        {
            return Err(ContractError::UnauthorizedAddr);
        }
        
        if info.funds[0].denom.clone() != permission.token_address
        {
            return Err(ContractError::UnauthorizedToken);
        }

        let (denom, amount) = (info.funds[0].denom.clone(), info.funds[0].amount);

        Ok(Response::new()
            .add_attribute("action", "add token")
            .add_message(BankMsg::Send {
                to_address: get_contract_address(_env)?,
                amount: vec![Coin { denom, amount }],
            }))
    }

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ContractVaultInfo => to_json_binary(&query::contract_vault_info(_deps,_env)?),
        QueryMsg::IsTransactionOpen => to_json_binary(&query::get_transaction_response(_deps,_env)?), 
        QueryMsg::GetNftOwner { contract_addr, token_id } 
        => to_json_binary(&query::get_nft_owner(_deps,_env, contract_addr, token_id)?),
    }
}

pub mod query {
    use cosmwasm_std::Addr;

    use crate::{helpers, msg::GetNftOwnerRespone};

    use super::*;

    pub fn get_nft_owner(_deps: Deps, _env: Env, contract_addr: String, token_id: String) -> StdResult<GetNftOwnerRespone>{
        match helpers::query_owner_of(_deps, Addr::unchecked(contract_addr), token_id) {
            Ok(owner_name) => Ok(GetNftOwnerRespone{
                owner_address: owner_name.owner
            }),
            Err(_) => todo!(),
        }
    }

    pub fn contract_vault_info(_deps: Deps, _env: Env) -> StdResult<GetNftOwnerRespone>{
        todo!()
    }
    
    pub fn get_transaction_response(_deps: Deps, _env: Env) -> StdResult<GetNftOwnerRespone>{
        todo!()
    }
    
    /*
        TODO:
            Check remaining tokens
            Check remaining nft
    */
}