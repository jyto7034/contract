pub mod executer {
    use crate::error::ContractError;
    use crate::helpers;
    use crate::state::{CONTRACT_CONFIG, ContractConfig, TransactionInfo};
    use cosmwasm_std::Addr;
    #[cfg(not(feature = "library"))]
    use cosmwasm_std::{coin, Deps, Env, MessageInfo, Response};

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
        if res.owner == helpers::get_contract_address(env.clone())? {
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
                helpers::execute_transfer_nft(
                    config.nft_contract_address.clone(),
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
            return Err(ContractError::UnauthorizedToken);
        }
        
        // sender 가 부족한 토큰을 보낸 경우.
        if info.funds[0].amount != config.exchange_rate {
            return Err(ContractError::Balanceinsufficient);
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

    pub fn approve_transaction_token_to_nft(
        config: &ContractConfig,
        trans_info: &TransactionInfo,
        token: String,
    ) -> Result<Response, ContractError>{
        helpers::execute_transfer_nft(config.nft_contract_address.clone(), trans_info.buyer.clone().into_string(), token)?;
        Ok(Response::new())
    }
    
    pub fn approve_transaction_nft_to_token(
        config: &ContractConfig,
        trans_info: &TransactionInfo
    )-> Result<Response, ContractError>{
        helpers::send_tokens(trans_info.buyer.clone(), vec![coin(config.exchange_rate.into(), config.token_address.clone())], "send_token");
        Ok(Response::new())
    }
}