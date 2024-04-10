#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::executer::executer;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ContractConfig, Product, CONTRACT_CONFIG, TRANSACTIONS_MAP};
use crate::state::{TransactionInfo, LOCK};

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
    if msg.expiration_block > 200000 {
        return Err(ContractError::HighBlock);
    }

    let config = ContractConfig {
        token_address: msg.token_address,
        admin: deps.api.addr_validate(info.sender.as_str())?,
        nft_contract_address: deps.api.addr_validate(msg.nft_contract_address.as_str())?,
        exchange_rate: Uint128::from(msg.exchange_rate),
        expiration_block: msg.expiration_block,
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONTRACT_CONFIG.save(deps.storage, &config)?;
    LOCK.save(deps.storage, &false)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // 거래 생성.
        ExecuteMsg::CreateTransaction {
            seller,
            desired_item,
            nft_token_id,
        } => execute::create_transaction(deps, env, info, seller, desired_item, nft_token_id),

        ExecuteMsg::ApproveTransaction { buyer } => {
            execute::approve_transaction(deps, env, info, buyer)
        }

        ExecuteMsg::SendNft { token_id, address } => {
            execute::send_nft(deps.as_ref(), info, env, token_id, address)
        }

        ExecuteMsg::SendToken { amount, address } => {
            execute::send_token(deps.as_ref(), info, env, amount, address)
        }

        ExecuteMsg::Refund { buyer } => execute::refund(deps, info, env, buyer),

        ExecuteMsg::Lock {} => execute::lock(deps, info),
    }
}

pub mod execute {
    use self::executer::transfer_nft;
    use crate::helpers;
    use crate::state::{EProduct, LOCK, RESERVED_NFT};
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
    ) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        // admin 권한 확인
        if info.sender.clone() == config.admin {
            Ok(helpers::send_tokens(
                Addr::unchecked(address),
                coins(amount as u128, config.token_address.clone()),
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

    pub fn refund(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        buyer: String,
    ) -> Result<Response, ContractError> {
        helpers::is_lock(&deps)?;
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        let trans_info = TRANSACTIONS_MAP.load(deps.storage, Addr::unchecked(buyer))?;

        if helpers::is_expired(env.block.height, trans_info.end_block)
            || info.sender.clone() == trans_info.buyer
            || info.sender.clone() == trans_info.seller
        {
            return match trans_info.product.to_enum {
                EProduct::NFT => executer::refund_token_to_nft(deps, &config, &trans_info),
                EProduct::TOKEN => executer::refund_nft_to_token(deps, &config, &trans_info),
            };
        } else {
            return Err(ContractError::NotExpired);
        };
    }

    pub fn approve_transaction(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        buyer: String,
    ) -> Result<Response, ContractError> {
        helpers::is_lock(&deps)?;
        let buyer = deps.api.addr_validate(buyer.as_str())?;
        let tran_config = TRANSACTIONS_MAP.load(deps.storage, buyer.clone());

        // 해당 거래가 존재하는지
        if let Err(_) = tran_config {
            return Err(ContractError::NoTransaction);
        }

        let tran_config = tran_config.unwrap();
        let config = CONTRACT_CONFIG.load(deps.storage)?;

        // 거래가 아직 유효한지 확인
        if helpers::is_expired(env.block.height, tran_config.end_block) {
            return Err(ContractError::Expired);
        }

        // buyer 가 지정한 판매자가 맞는지 확인
        if tran_config.seller != info.sender {
            return Err(ContractError::NotDesignatedSeller);
        }

        // nft <-> token 거래 응답 시 seller 는 contract 에게 funds 입력해야함.
        if tran_config.product.to_enum == EProduct::TOKEN {
            // --amount 를 통해 들어온 자금이 없을 때.
            if info.funds.is_empty() {
                return Err(ContractError::NotReceivedFunds);
            }

            // funds 의 자금이 교환 비율과 맞지 않을 때.
            if info.funds[0].amount != config.exchange_rate {
                return Err(ContractError::NotMatchExchangeRate);
            }

            // sender 가 잘못된 토큰을 보낸 경우.
            if info.funds[0].denom.clone() != config.token_address {
                helpers::send_tokens(info.sender.clone(), info.funds, "refund");
                return Err(ContractError::UnauthorizedToken);
            }
        }

        match &tran_config.product.to_enum {
            EProduct::NFT => {
                executer::approve_transaction_token_to_nft(deps, &config, &tran_config)
            }
            EProduct::TOKEN => {
                executer::approve_transaction_nft_to_token(deps, env, &config, &tran_config)
            }
        }
        // Ok(Response::new())
    }

    // 판매자가 응답할 거래 트랜잭션 생성
    pub fn create_transaction(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        seller: String,
        desired_item: String,
        token_id: String,
    ) -> Result<Response, ContractError> {
        helpers::is_lock(&deps)?;
        let transaction_info = TRANSACTIONS_MAP.load(deps.storage, info.sender.clone());
        let config = CONTRACT_CONFIG.load(deps.storage)?;

        // 한 사람이 여러 transaction 을 생성 못하도록 막음.
        if let Ok(_) = transaction_info {
            return Err(ContractError::TransactionAlreadyProgress);
        }

        // 두 사람이 동시에 한 nft 를 지정 못하도록 막음.
        let res = RESERVED_NFT.load(deps.storage, token_id.clone());
        if let Ok(_) = res {
            return Err(ContractError::ReservedNFT);
        }

        // seller 는 info.sender 가 될 수 없음.
        if seller == info.sender {
            return Err(ContractError::BadTransaction);
        }

        let transaction_info = TransactionInfo {
            seller: deps.api.addr_validate(seller.as_str())?,
            buyer: info.sender.clone(),
            start_block: env.block.height,
            end_block: env.block.height + config.expiration_block,
            product: Product::new(desired_item.as_str(), token_id.clone()),
        };

        // token 으로 nft 구매 할 때만 funds 확인
        if transaction_info.product.to_enum == EProduct::NFT {
            // --amount 를 통해 들어온 자금이 없을 때.
            if info.funds.is_empty() {
                return Err(ContractError::NotReceivedFunds);
            }

            // funds 의 자금이 교환 비율과 맞지 않을 때.
            if info.funds[0].amount != config.exchange_rate {
                return Err(ContractError::NotMatchExchangeRate);
            }

            // sender 가 잘못된 토큰을 보낸 경우.
            if info.funds[0].denom.clone() != config.token_address {
                helpers::send_tokens(info.sender.clone(), info.funds, "refund");
                return Err(ContractError::UnauthorizedToken);
            }
        }

        TRANSACTIONS_MAP.save(deps.storage, info.sender.clone(), &transaction_info)?;
        match &transaction_info.product.to_enum {
            EProduct::NFT => executer::create_transaction_token_to_nft(
                deps,
                info,
                env,
                transaction_info.product.token_id.clone(),
            ),
            EProduct::TOKEN => executer::create_transaction_nft_to_token(
                deps.as_ref(),
                info,
                env,
                transaction_info.product.token_id.clone(),
            ),
        }
        // Ok(Response::new())
    }
}

pub enum QueryProductType {
    BUYER(String),
    TOKEN(String),
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTransactionInfo { buyer, token } => match (buyer, token) {
            (None, Some(_token)) => to_json_binary(&query::get_transaction_info(
                deps,
                _env,
                QueryProductType::TOKEN(_token),
            )),
            (Some(_buyer), None) => to_json_binary(&query::get_transaction_info(
                deps,
                _env,
                QueryProductType::BUYER(_buyer),
            )),
            _ => {
                panic!("Query Error")
            }
        },
    }
}

pub mod query {
    use crate::{msg::TransactionResponse, state::RESERVED_NFT};

    use super::*;

    pub fn get_transaction_info(
        deps: Deps,
        _env: Env,
        pro: QueryProductType,
    ) -> TransactionResponse {
        let dummy = TransactionResponse {
            desired_nft: "".to_string(),
            seller: "".to_string(),
            buyer: "".to_string(),
            start_expiration: 0,
            end_expiration: 0,
            product: Product::new("nft", "".to_string()),
        };
    
        let buyer_addr_result = match pro {
            QueryProductType::BUYER(buyer) => deps.api.addr_validate(&buyer),
            QueryProductType::TOKEN(token) => RESERVED_NFT.load(deps.storage, token)
                .and_then(|buyer| deps.api.addr_validate(&buyer)),
        };
    
        let buyer_addr = match buyer_addr_result {
            Ok(addr) => addr,
            Err(_) => return dummy,
        };
    
        let trans_config = match TRANSACTIONS_MAP.load(deps.storage, buyer_addr.clone()) {
            Ok(config) => config,
            Err(_) => return dummy,
        };
    
        TransactionResponse {
            desired_nft: trans_config.product.token_id.clone(),
            seller: trans_config.seller.to_string(),
            buyer: buyer_addr.to_string(),
            start_expiration: trans_config.start_block,
            end_expiration: trans_config.end_block,
            product: trans_config.product.clone(),
        }
    }
}
