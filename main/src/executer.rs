pub mod executer {
    use crate::error::ContractError;
    use crate::helpers::{self, TransferNft};
    use crate::state::{
        ContractConfig, TransactionInfo, CONTRACT_CONFIG, RESERVED_NFT, TRANSACTIONS_MAP,
    };
    #[cfg(not(feature = "library"))]
    use cosmwasm_std::{coin, Deps, Env, MessageInfo, Response};
    use cosmwasm_std::{coins, Addr, BankMsg, DepsMut};

    // nft 를 token 으로 교환.
    pub fn create_transaction_nft_to_token(
        deps: Deps,
        _info: MessageInfo,
        _env: Env,
        income_nft: String,
    ) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;

        // sender 가 잘못된 nft 를 보낸 경우.
        // 존재하지 않는 nft 을 보내는 경우 Wasm runtime Failed 뜸.
        let res = helpers::query_owner_of(
            deps,
            config.nft_contract_address.clone(),
            income_nft.clone(),
        )?;

        // 해당 nft 가 owner 것인지 확인
        if res.owner != _info.sender {
            return Err(ContractError::DoesNotOwnNFT);
        }

        transfer_nft(deps, helpers::get_contract_address(_env)?, income_nft)
    }

    // token 으로 nft 를 사는 경우
    pub fn create_transaction_token_to_nft(
        deps: DepsMut,
        info: MessageInfo,
        _env: Env,
        wanted_nft: String,
    ) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        let trans_info = TRANSACTIONS_MAP.load(deps.storage, info.sender.clone())?;

        // 구매 하고자 하는 nft 가 유효한가?
        let res = helpers::query_owner_of(
            deps.as_ref(),
            config.nft_contract_address.clone(),
            wanted_nft.clone(),
        )?;

        if res.owner != trans_info.seller.clone() {
            return Err(ContractError::DoesNotOwnNFT);
        }

        // 예약된 nft 인지 확인.
        let res = RESERVED_NFT.load(deps.storage, wanted_nft);
        if let Ok(_) = res{
            return Err(ContractError::AlreadyInTransaction);
        }

        RESERVED_NFT.save(
            deps.storage,
            trans_info.product.token_id.clone(),
            &trans_info.buyer.clone().to_string(),
        )?;

        // CheckList
        //  funds 를 통해 contract 에 자금이 들어와있긴 한데,
        //  기억상 contract 에 token 이 없다고 떴었던 것 같음.
        Ok(Response::new())
    }

    pub fn transfer_nft(
        deps: Deps,
        buyer: String,
        token_id: String,
    ) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        let core_msg = TransferNft {
            recipient: buyer,
            token_id,
        };
        let processed_msg = core_msg
            .clone()
            .into_cosmos_msg(config.nft_contract_address)?;
        Ok(Response::new().add_message(processed_msg))
    }

    pub fn approve_transaction_token_to_nft(
        deps: DepsMut,
        config: &ContractConfig,
        trans_info: &TransactionInfo,
    ) -> Result<Response, ContractError> {
        // seller 가 nft 를 가지고 있는지 확인
        let res = helpers::query_owner_of(
            deps.as_ref(),
            config.nft_contract_address.clone(),
            trans_info.product.token_id.clone(),
        )?;
        if res.owner != trans_info.seller {
            return Err(ContractError::DoesNotOwnNFT);
        }

        // 거래 내역 및 nft 예약 삭제.
        TRANSACTIONS_MAP.remove(deps.storage, trans_info.buyer.clone());
        RESERVED_NFT.remove(deps.storage, trans_info.product.token_id.to_string());

        // NFT 전송
        let core_msg = TransferNft {
            recipient: trans_info.buyer.clone().into_string(),
            token_id: trans_info.product.token_id.clone(),
        };
        let processed_msg = core_msg
            .clone()
            .into_cosmos_msg(config.nft_contract_address.clone())?;

        Ok(Response::new()
            .add_message(processed_msg)
            .add_message(BankMsg::Send {
                to_address: deps
                    .api
                    .addr_validate(trans_info.seller.as_str())?
                    .to_string(),
                amount: coins(config.exchange_rate.u128(), config.token_address.clone()),
            }))
    }

    pub fn approve_transaction_nft_to_token(
        deps: DepsMut,
        env: Env,
        config: &ContractConfig,
        trans_info: &TransactionInfo,
    ) -> Result<Response, ContractError> {
        // contract 에 들어와있는 token 확인
        let contract_address = Addr::unchecked(helpers::get_contract_address(env)?);
        let balances = helpers::query_balance(deps.as_ref(), contract_address.clone())?;
        if balances.is_empty() {
            return Err(ContractError::BalanceQueryFailed);
        }

        // contract wallet 에서 교환용 token 찾기
        let wallet = balances
            .iter()
            .find(|wallet| wallet.denom == config.token_address);

        if let Some(wallet) = wallet {
            if wallet.amount < config.exchange_rate {
                return Err(ContractError::Balanceinsufficient);
            } else {
                // nft 를 가지고 있는 contract 의 approvals 에 escrow contract 가 포함되어 있어야함.
                let core_msg = TransferNft {
                    recipient: trans_info.seller.to_string(),
                    token_id: trans_info.product.token_id.clone(),
                };
                let processed_msg = core_msg
                    .clone()
                    .into_cosmos_msg(config.nft_contract_address.clone())?;

                // TRANSACTIONS_MAP 에서 해당 거래를 지워야함.
                TRANSACTIONS_MAP.remove(deps.storage, trans_info.buyer.clone());

                Ok(Response::new()
                    .add_message(processed_msg)
                    .add_message(BankMsg::Send {
                        to_address: trans_info.buyer.clone().to_string(),
                        amount: vec![coin(
                            config.exchange_rate.into(),
                            config.token_address.clone(),
                        )],
                    }))
            }
        } else {
            return Err(ContractError::DoesNotHaveToken);
        }
    }

    pub fn refund_token_to_nft(
        deps: DepsMut,
        config: &ContractConfig,
        trans_info: &TransactionInfo,
    ) -> Result<Response, ContractError> {
        // TRANSACTIONS_MAP 에서 해당 거래를 지워야함.
        TRANSACTIONS_MAP.remove(deps.storage, trans_info.buyer.clone());
        RESERVED_NFT.remove(
            deps.storage,
            trans_info.product.token_id.clone().to_string(),
        );
        // 토큰 환불
        Ok(helpers::send_tokens(
            trans_info.buyer.clone(),
            coins(config.exchange_rate.u128(), config.token_address.clone()),
            "refund",
        ))
    }

    pub fn refund_nft_to_token(
        deps: DepsMut,
        _config: &ContractConfig,
        trans_info: &TransactionInfo,
    ) -> Result<Response, ContractError> {
        // TRANSACTIONS_MAP 에서 해당 거래를 지워야함.
        TRANSACTIONS_MAP.remove(deps.storage, trans_info.buyer.clone());
        // nft 환불
        transfer_nft(
            deps.as_ref(),
            trans_info.buyer.to_string(),
            trans_info.product.token_id.clone(),
        )
    }
}
