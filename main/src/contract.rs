#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::executer::executer;
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
        // 거래 생성.
        ExecuteMsg::CreateTransaction {
            seller,
            desired_item,
            nft_token_id,
        } => execute::create_transaction(deps, env, info, seller, desired_item, nft_token_id, Some(false)),

        // 구매자 거래 수락. ( 해당 토큰에 대해서 approve 가 되어있어야함. )
        ExecuteMsg::ApproveTransaction { buyer } => {
            execute::approve_transaction(deps, env, info, buyer)
        }

        ExecuteMsg::SendNft { token_id, address } => { execute::send_nft(deps.as_ref(), info, env, token_id,  address) },

        ExecuteMsg::SendToken { amount, address } => { execute::send_token(deps.as_ref(), info, env, amount,  address) },

        // 거래 실패 시, 자금 반환.
        ExecuteMsg::Refund { buyer } => execute::refund(deps, info, env, buyer)
    }
}

pub mod execute {
    use cosmwasm_std::{coins, Addr, BankMsg, Coin};
    use crate::helpers;
    use crate::state::{EProduct, RESERVED_NFT};
    use self::executer::transfer_nft;

    use super::*;

    pub fn send_token(
        deps: Deps,
        info: MessageInfo,
        _env: Env,
        amount: u64,
        address: String,
    )-> Result<Response, ContractError>{
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        // admin 권한 확인
        if info.sender.clone() == config.admin{
            Ok(helpers::send_tokens(Addr::unchecked(address), coins(amount as u128, config.token_address.clone()), "send"))
        }else{
            Err(ContractError::Unauthorized)
        }
    }
    
    pub fn send_nft(
        deps: Deps,
        info: MessageInfo,
        _env: Env,
        token_id: String,
        address: String,
    )-> Result<Response, ContractError>{
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        // admin 권한 확인
        if info.sender.clone() == config.admin{
            transfer_nft(deps, address, token_id)
        }else{
            Err(ContractError::Unauthorized)
        }
    }

    pub fn refund(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        buyer: String,
    ) -> Result<Response, ContractError> {
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
        let buyer = deps.api.addr_validate(buyer.as_str())?;
        let tran_config = TRANSACTIONS_MAP.load(deps.storage, buyer.clone());
        if let Err(_) = tran_config{
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
        test: Option<bool>,
    ) -> Result<Response, ContractError> {
        // TODO!
        // --amount 로 이미 들어왔기 때문에, err 시 반환해줘야함.
        let transaction_info = TRANSACTIONS_MAP.load(deps.storage, info.sender.clone());
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        
        // 한 사람이 여러 transaction 을 생성 못하도록 막음.
        if let Ok(_) = transaction_info {
            return Err(ContractError::TransactionAlreadyProgress);
        }

        // TODO: RESERVED_NFT 삭제해야함.
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
                deps.as_ref(),
                info,
                env,
                transaction_info.product.token_id.clone(),
                test.unwrap(),
            ),
            EProduct::TOKEN => executer::create_transaction_nft_to_token(
                deps.as_ref(),
                info,
                env,
                transaction_info.product.token_id.clone(),
                test.unwrap(),
            ),
        }
        // Ok(Response::new())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTransactionInfo { buyer } => {
            to_json_binary(&query::get_transaction_info(deps, _env, buyer))
        }
    }
}

pub mod query {
    use crate::msg::TransactionResponse;

    use super::*;

    pub fn get_transaction_info(deps: Deps, _env: Env, buyer: String) -> TransactionResponse {
        let buyer = deps.api.addr_validate(buyer.as_str()).unwrap();

        // 존재하지 않는 경우 처리해야함.
        let tran_config = match TRANSACTIONS_MAP.load(deps.storage, buyer.clone()) {
            Ok(val) => val,
            Err(_) => {
                return TransactionResponse {
                    desired_nft: "".to_string(),
                    seller: "".to_string(),
                    buyer: "".to_string(),
                    start_expiration: 0,
                    end_expiration: 0,
                    product: Product::new("nft", "".to_string()),
                }
            }
        };

        TransactionResponse {
            desired_nft: tran_config.product.token_id.clone(),
            seller: tran_config.seller.to_string(),
            buyer: buyer.to_string(),
            start_expiration: tran_config.start_block,
            end_expiration: tran_config.end_block,
            product: tran_config.product.clone(),
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
