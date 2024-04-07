pub mod executer {
    use crate::error::ContractError;
    use crate::helpers::{self, TransferNft};
    use crate::state::{ContractConfig, TransactionInfo, CONTRACT_CONFIG, TRANSACTIONS_MAP};
    #[cfg(not(feature = "library"))]
    use cosmwasm_std::{coin, Deps, Env, MessageInfo, Response};
    use cosmwasm_std::{coins, Addr, DepsMut};

    // nft 를 token 으로 교환.
    pub fn create_transaction_nft_to_token(
        deps: Deps,
        _info: MessageInfo,
        env: Env,
        income_nft: String,
    ) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;

        // sender 가 잘못된 nft 를 보내는 경우.
        // 존재하지 않는 nft 을 보내는 경우 Wasm runtime Failed 뜸.
        let res = helpers::query_owner_of(
            deps,
            config.nft_contract_address.clone(),
            income_nft.clone(),
        )?;
        // 해당 nft 가 owner 것인지 확인
        if res.owner == _info.sender {
            return Err(ContractError::DoesNotOwnNFT);
        }

        // contract 의 token 이 적당한가?
        let contract_address = Addr::unchecked(helpers::get_contract_address(env)?);
        let balances = helpers::query_balance(deps, contract_address.clone())?;
        if balances.is_empty() {
            return Err(ContractError::BalanceQueryFailed);
        }

        // wallet 에서 교환용 token 찾기
        let wallet = balances
            .iter()
            .find(|wallet| wallet.denom == config.token_address);
        if let Some(wallet) = wallet {
            if wallet.amount < config.exchange_rate {
                return Err(ContractError::Balanceinsufficient);
            } else {
                // nft 를 가지고 있는 contract 의 approvals 에 escrow contract 가 포함되어 있어야함.
                // nft 를 contract 로 전송함.
                transfer_nft(
                    deps,
                    contract_address.to_string(),
                    income_nft,
                )?;

                return Ok(
                    Response::new().add_attribute("create_transaction_nft_to_token", "Created")
                );
            }
        } else {
            return Err(ContractError::DoesNotHaveToken);
        }
    }

    // token 으로 nft 를 사는 경우
    pub fn create_transaction_token_to_nft(
        deps: Deps,
        info: MessageInfo,
        env: Env,
        wanted_nft: String,
    ) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;

        // sender 가 잘못된 토큰을 보낸 경우.
        if info.funds[0].denom.clone() != config.token_address {
            helpers::send_tokens(info.sender.clone(), info.funds, "refund");
            return Err(ContractError::UnauthorizedToken);
        }

        // sender 가 부족한 토큰을 보낸 경우.
        if info.funds[0].amount != config.exchange_rate {
            helpers::send_tokens(info.sender.clone(), info.funds, "refund");
            return Err(ContractError::BadFunds);
        }

        // 구매 하고자 하는 nft 가 유효한가?
        let res = helpers::query_owner_of(
            deps,
            config.nft_contract_address.clone(),
            wanted_nft.clone(),
        )?;
        if res.owner == helpers::get_contract_address(env.clone())? {
            return Err(ContractError::DoesNotOwnNFT);
        }

        Ok(Response::new().add_attribute("create_transaction_token_to_nft", "Created"))
    }

    pub fn transfer_nft(deps: Deps, buyer: String, token_id: String) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;
        let core_msg = TransferNft {
            recipient: buyer,
            token_id,
         };
         let processed_msg = core_msg
         .clone()
         .into_cosmos_msg(config.nft_contract_address)?;
         Ok(Response::new()
         .add_message(processed_msg))

        // helpers::transfer_nft(config.nft_contract_address, buyer, "1".to_string())?;
        // Ok(Response::new())
    }

    pub fn approve_transaction_token_to_nft(
        deps: DepsMut,
        config: &ContractConfig,
        trans_info: &TransactionInfo,
    ) -> Result<Response, ContractError> {
        let core_msg = TransferNft {
            recipient: trans_info.buyer.clone().into_string(),
            token_id: trans_info.product._token.clone(),
         };
         let processed_msg = core_msg
         .clone()
         .into_cosmos_msg(config.nft_contract_address.clone())?;

        // TRANSACTIONS_MAP 에서 해당 거래를 지워야함.
        TRANSACTIONS_MAP.remove(deps.storage, trans_info.buyer.clone());
        
         Ok(Response::new()
         .add_message(processed_msg))
        // let res = transfer_nft(
        //     deps,
        //     trans_info.buyer.clone().into_string(),
        //     token,
        // )?;
        // Ok(res)
    }

    pub fn approve_transaction_nft_to_token(
        deps: DepsMut,
        config: &ContractConfig,
        trans_info: &TransactionInfo,
    ) -> Result<Response, ContractError> {
        helpers::send_tokens(
            trans_info.buyer.clone(),
            vec![coin(
                config.exchange_rate.into(),
                config.token_address.clone(),
            )],
            "send_token",
        );
        // TRANSACTIONS_MAP 에서 해당 거래를 지워야함.
        TRANSACTIONS_MAP.remove(deps.storage, trans_info.buyer.clone());
        Ok(Response::new())
    }

    pub fn refund_token_to_nft(
        config: &ContractConfig,
        trans_info: &TransactionInfo,
    ) -> Result<Response, ContractError> {
        // 토큰 환불
        helpers::send_tokens(
            trans_info.buyer.clone(),
            coins(config.exchange_rate.u128(), config.token_address.clone()),
            "refund",
        );
        Ok(Response::new())
    }

    pub fn refund_nft_to_token(
        deps: Deps,
        _config: &ContractConfig,
        trans_info: &TransactionInfo,
    ) -> Result<Response, ContractError> {
        // nft 환불
        transfer_nft(
            deps,
            trans_info.buyer.to_string(),
            trans_info.product._token.clone(),
        )?;

        Ok(Response::new())
    }
}
