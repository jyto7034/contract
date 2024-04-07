#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::executer::executer;
use crate::helpers::get_contract_address;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::TransactionInfo;
use crate::state::{ContractConfig, Product, CONTRACT_CONFIG, TRANSACTIONS_MAP};

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
    /*
              token_address : escrow 에서 사용할 token 의 addr
                      admin : escrow admin
       NFT_contract_address : NFT Contract 주소
              exchange_rate : 교환 비율
    */
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
        ExecuteMsg::AddTokenToContract {} => execute::add_token_to_contract(deps.as_ref(), env, info),
        // 거래 생성.
        ExecuteMsg::CreateTransaction {
            seller,
            desired_item,
            nft_token_id,
        } => execute::create_transaction(deps, env, info, seller, desired_item, nft_token_id),

        // 구매자 거래 수락. ( 해당 토큰에 대해서 approve 가 되어있어야함. )
        ExecuteMsg::ApproveTransaction { buyer } => {
            execute::transaction_approve(deps, env, info, buyer)
        },

        // 거래 실패 시, 자금 반환.
        ExecuteMsg::Refund { buyer } => execute::refund(deps.as_ref(), env, buyer),
    }
}

pub mod execute {
    use cosmwasm_std::Addr;

    use crate::state::RESERVED_NFT;
    use crate::helpers::{self, TransferNft};

    use super::*;

    pub fn transfer_nft(deps: Deps, buyer: String, token_id: String) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        let core_msg = TransferNft {
            recipient: buyer,
            token_id,
         };
         // use our impl'ed functions to make it a serialized CosmosMsg
         let processed_msg = core_msg
         .clone()
         .into_cosmos_msg(config.nft_contract_address)?;
         // attach the message to the response so that it gets sent
         Ok(Response::new()
         .add_message(processed_msg))

        // helpers::transfer_nft(config.nft_contract_address, buyer, "1".to_string())?;
        // Ok(Response::new())
    }

    pub fn refund(deps: Deps, env: Env, buyer: String) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        let trans_info = TRANSACTIONS_MAP.load(deps.storage, Addr::unchecked(buyer))?;

        if helpers::is_expired(env.block.height, trans_info.end_block) {
            match trans_info.product {
                Product::NFT(_) => executer::refund_token_to_nft(&config, &trans_info)?,
                Product::TOKEN(_) => executer::refund_nft_to_token(deps, &config, &trans_info)?,
                Product::NONE => return Err(ContractError::UnknownError),
            };
            return Ok(Response::new());
        }
        return Err(ContractError::NotExpired);
    }

    pub fn transaction_approve(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        buyer: String,
    ) -> Result<Response, ContractError> {
        let buyer = deps.api.addr_validate(buyer.as_str())?;
        let tran_config = TRANSACTIONS_MAP.load(deps.storage, buyer.clone())?;
        let config = CONTRACT_CONFIG.load(deps.storage)?;

        // 거래가 아직 유효한지 확인
        if helpers::is_expired(env.block.height, tran_config.end_block) {
            return Err(ContractError::Expired);
        }

        // buyer 가 지정한 판매자가 맞는지 확인
        if tran_config.seller != info.sender {
            return Err(ContractError::NotDesignatedSeller);
        }

        match &tran_config.product {
            Product::NFT(token_id) => {
                executer::approve_transaction_token_to_nft(deps.as_ref(), &config, &tran_config, token_id.clone())?
            }
            Product::TOKEN(_) => executer::approve_transaction_nft_to_token(&config, &tran_config)?,
            Product::NONE => return Err(ContractError::UnknownError),
        };

        // TRANSACTIONS_MAP 에서 해당 거래를 지워야함.
        TRANSACTIONS_MAP.remove(deps.storage, buyer);

        Ok(Response::new())
    }

    // 구매자가 판매자가 응답할 거래 트랜잭션 생성
    pub fn create_transaction(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        seller: String,
        desired_item: String,
        token_id: String,
    ) -> Result<Response, ContractError> {
        let transaction_info = TRANSACTIONS_MAP.load(deps.storage, info.sender.clone());
        let config = CONTRACT_CONFIG.load(deps.storage)?;

        // 한 사람이 여러 transaction 을 생성 못하도록 막음.
        if let Ok(_) = transaction_info {
            return Err(ContractError::TransactionAlreadyProgress);
        }

        // 두 사람이 동시에 한 nft 를 지정 못하도록 막음.
        let reserved_nft = RESERVED_NFT
            .iter(deps.storage)?
            .find(|item| item == &Ok(token_id.clone()));
        if let Some(_) = reserved_nft {
            return Err(ContractError::ReservedNFT);
        }

        // seller 는 info.sender 가 될 수 없음.
        if seller == info.sender {
            return Err(ContractError::BadTransaction);
        }

        // --amount 를 통해 들어온 자금이 없을 때.
        if info.funds.is_empty() {
            return Err(ContractError::NotReceivedFunds);
        }

        // funds 의 자금이 교환 비율과 맞지 않을 때.
        if info.funds[0].amount != config.exchange_rate {
            return Err(ContractError::NotMatchExchangeRate);
        }

        let transaction_info = TransactionInfo {
            seller: deps.api.addr_validate(seller.as_str())?,
            buyer: info.sender.clone(),
            start_block: env.block.height,
            end_block: env.block.height + config.expiration_block,
            product: Product::new(desired_item, token_id),
        };

        TRANSACTIONS_MAP.save(deps.storage, info.sender.clone(), &transaction_info)?;
        match &transaction_info.product {
            Product::NFT(token) => {
                executer::create_transaction_nft_to_token(deps.as_ref(), info, env, token.clone())
            }
            Product::TOKEN(token) => {
                executer::create_transaction_token_to_nft(deps.as_ref(), info, env, token.clone())
            }
            Product::NONE => Err(ContractError::UnauthorizedNft),
        }
    }

    // Contract 에 token 충전
    pub fn add_token_to_contract(
        deps: Deps,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        // admin 인지 확인
        if info.sender != config.admin {
            return Err(ContractError::UnauthorizedAddr);
        }

        // --amount 를 통해 들어온 자금이 없을 때.
        if info.funds.is_empty() {
            return Err(ContractError::NotReceivedFunds);
        }

        // funds 의 자금이 0 일 때.
        if info.funds[0].amount.u128() == 0 {
            return Err(ContractError::NoFunds);
        }

        let config = CONTRACT_CONFIG.load(deps.storage)?;

        // sender 의 주소가 admin 권한이 아닌 경우 혹은 잘못된 토큰을 보내고 있는 경우.
        if info.sender.into_string() != config.admin {
            return Err(ContractError::UnauthorizedAddr);
        }

        if info.funds[0].denom.clone() != config.token_address {
            return Err(ContractError::UnauthorizedToken);
        }

        let (denom, amount) = (info.funds[0].denom.clone(), info.funds[0].amount);

        Ok(Response::new()
            .add_attribute("action", "add token")
            .add_message(BankMsg::Send {
                to_address: get_contract_address(env)?,
                amount: vec![Coin { denom, amount }],
            }))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTransactionInfo { buyer } => {
            to_json_binary(&query::get_transaction_info(deps, env, buyer))
        }
    }
}

pub mod query {
    use crate::msg::TransactionResponse;

    use super::*;

    pub fn get_transaction_info(deps: Deps, _env: Env, buyer: String) -> TransactionResponse {
        let buyer = deps.api.addr_validate(buyer.as_str()).unwrap();

        // 존재하지 않는 경우 처리해야함.
        let tran_config = match TRANSACTIONS_MAP.load(deps.storage, buyer.clone()){
            Ok(val) => val,
            Err(_) => return TransactionResponse{
                desired_nft: "".to_string(),
                seller: "".to_string(),
                buyer: "".to_string(),
                start_expiration: 0,
                end_expiration: 0,
            },
        };

        TransactionResponse {
            desired_nft: tran_config.product.get_nft_token().unwrap(),
            seller: tran_config.seller.to_string(),
            buyer: buyer.to_string(),
            start_expiration: tran_config.start_block,
            end_expiration: tran_config.end_block,
        }
    }
}

// pub fn get_nft_owner(
//     deps: Deps,
//     contract_addr: String,
//     token_id: String,
// ) -> StdResult<GetNftOwnerRespone> {
//     match helpers::query_owner_of(deps, Addr::unchecked(contract_addr), token_id) {
//         Ok(owner_name) => Ok(GetNftOwnerRespone {
//             owner_address: owner_name.owner,
//         }),
//         Err(_) => todo!(),
//     }
// }
