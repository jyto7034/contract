#[cfg(test)]
mod tests {
    use crate::msg::ExecuteMsg;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{coins, Addr, Empty};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            // 이는 특정 주소에 대하여 자금을 설정하는 단계이다.
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(ADMIN),
                    coins(100u128, NATIVE_DENOM.to_string()),
                )
                .unwrap();
        })
    }

    mod contract {
        use cosmwasm_std::Uint128;

        use super::*;

        #[test]
        fn proper_instantiate() {
            let mut app = mock_app();
            let cw_template_id = app.store_code(contract_template());

            let msg = InstantiateMsg {
                token_address: NATIVE_DENOM.to_string(),
                nft_contract_address: "".to_string(),
                exchange_rate: Uint128::new(800000),
            };

            // contract 를 인스턴스화 시키는 함수
            let cw_template_contract_addr = app
                .instantiate_contract(
                    cw_template_id,
                    Addr::unchecked(ADMIN),
                    &msg,
                    &[],
                    "test",
                    None,
                )
                .unwrap();

            // contract 에 ExecuteMsg 전송
            app.execute_contract(
                Addr::unchecked(ADMIN),
                cw_template_contract_addr.clone(),
                &ExecuteMsg::AddTokenToContract {},
                &coins(5, "denom"),
            )
            .unwrap();
        }

        #[test]
        fn create_transaction_nft_to_token() {}
    }
}
