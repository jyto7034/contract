pub mod executer {
    use crate::error::ContractError;
    use crate::helpers::{self, get_contract_address, TransferNft};
    use crate::state::{ContractConfig, CONTRACT_CONFIG};
    use cosmwasm_std::{coin, Addr, BankMsg};
    #[cfg(not(feature = "library"))]
    use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response};

    // nft 를 token 으로 교환.
    pub fn create_transaction_nft_to_token(
        deps: Deps,
        _info: MessageInfo,
        _env: Env,
        config: &ContractConfig,
        income_nft: String,
    ) -> Result<Response, ContractError> {
        let contract_address = Addr::unchecked(helpers::get_contract_address(_env.clone())?);

        // contract 의 token 확인.
        let balances = helpers::query_balance(deps, contract_address.clone())?;
        if balances.is_empty() {
            return Err(ContractError::BalanceQueryFailed);
        }

        let wallet = balances
            .iter()
            .find(|wallet| wallet.denom == config.token_address);
        if let Some(wallet) = wallet {
            if wallet.amount < config.exchange_rate {
                return Err(ContractError::Balanceinsufficient);
            } else {
                // 사용자가 소유한 nft 를 contract 로 전송
                // nft approvals 에 escrow contract 가 등록 되어있어야함. -> 사이트단에서 해결
                let core_msg = TransferNft {
                    recipient: get_contract_address(_env.clone()).unwrap(),
                    token_id: income_nft.clone(),
                };
                let processed_msg = core_msg
                    .clone()
                    .into_cosmos_msg(config.nft_contract_address.clone())?;

                Ok(Response::new()
                    .add_message(processed_msg)
                    .add_message(BankMsg::Send {
                        to_address: _info.sender.to_string(),
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

    // token 으로 nft 를 사는 경우
    pub fn create_transaction_token_to_nft(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        wanted_nft: String,
    ) -> Result<Response, ContractError> {
        let config = CONTRACT_CONFIG.load(deps.storage)?;

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
            return Err(ContractError::UnauthorizedToken);
        }

        // 구매 하고자 하는 nft 가 contract 의 nft 가 맞고 contract 가 소유 하고 있는가?
        let res = helpers::query_owner_of(
            deps.as_ref(),
            config.nft_contract_address.clone(),
            wanted_nft.clone(),
        )?;
        if res.owner != get_contract_address(env).unwrap() {
            return Err(ContractError::DoesNotOwnNFT);
        }

        // 모든 assert 를 충족한 경우
        // contract 의 nft 를 transaction 을 발생시킨 info.sender 에게 전송함.
        let core_msg = TransferNft {
            recipient: info.sender.to_string(),
            token_id: wanted_nft,
        };
        let processed_msg = core_msg
            .clone()
            .into_cosmos_msg(config.nft_contract_address.clone())?;

        Ok(Response::new().add_message(processed_msg))
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
}
