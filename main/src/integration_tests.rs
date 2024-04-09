#[cfg(test)]
mod tests {
    use crate::{helpers::CwTemplateContract, msg::{ExecuteMsg, InstantiateMsg}, state::{ContractConfig, Product, TransactionInfo}, ContractError};
    use anyhow::Error;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, AppResponse, Contract, ContractWrapper, Executor};

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";
    const NFT_CONTRACT: &str = "testContract";
    
    pub fn generate_contract_config() -> ContractConfig{
        ContractConfig{
            token_address: NATIVE_DENOM.to_string(),
            admin: Addr::unchecked(ADMIN),
            nft_contract_address: Addr::unchecked(NFT_CONTRACT),
            exchange_rate: Uint128::new(10),
            expiration_block: 100,
        }
    }

    pub fn generate_transaction_info() -> TransactionInfo{
        TransactionInfo{ 
            buyer: Addr::unchecked(USER),
            seller: Addr::unchecked(USER),
            start_block: 0, 
            end_block: 100, 
            product: Product::new("nft", NATIVE_DENOM.to_string())
        }
    }

    mod tests_tool{
        use super::*;
        pub fn contract_template() -> Box<dyn Contract<Empty>> {
            let contract = ContractWrapper::new(
                crate::contract::execute,
                crate::contract::instantiate,
                crate::contract::query,
            );
            Box::new(contract)
        }
    
        pub fn mock_app() -> App {
            AppBuilder::new().build(|router, _, storage| {
                router
                    .bank
                    .init_balance(
                        storage,
                        &Addr::unchecked(ADMIN),
                        vec![Coin::new(100, NATIVE_DENOM.to_string()), Coin::new(100, "error".to_string())],
                    )
                    .unwrap();
                router
                    .bank
                    .init_balance(
                        storage,
                        &Addr::unchecked(USER),
                        vec![Coin::new(100, NATIVE_DENOM.to_string()), Coin::new(100, "error".to_string())],
                    )
                    .unwrap();
            })
        }
    
        pub fn generate_instantiate_msg(config: ContractConfig) -> InstantiateMsg{
            InstantiateMsg {
                token_address: config.token_address.clone(),
                nft_contract_address: config.nft_contract_address.to_string(),
                exchange_rate: config.exchange_rate.u128() as u32,
                expiration_block: config.expiration_block,
            }
        }
    
        pub fn set_contract(config: ContractConfig) -> (App, CwTemplateContract){
            let mut app = mock_app();
            let cw_template_id = app.store_code(contract_template());
    
            let _msg = generate_instantiate_msg(config);
    
            let cont_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &_msg,
                &[],
                "test",
                None,
            )
            .unwrap();
    
            (app, CwTemplateContract(cont_addr))
        }
    
        pub fn should_panic(err: Result<AppResponse, Error>, expected: ContractError) {
            match err {
                Ok(_) => panic!("Expected error but got Ok response"),
                Err(e) => {
                    match e.downcast_ref::<ContractError>() {
                        Some(actual_err) => assert_eq!(*actual_err, expected, "Expected ContractError but got a different error type"),
                        None => panic!("What?"),
                    }
                }
            }
        }

        // pub fn assert_eq(err: Error, expected: ContractError) {
        //     match err.downcast_ref::<ContractError>() {
        //         Some(actual_err) => assert_eq!(*actual_err, expected, "Expected ContractError but got a different error type"),
        //         None => panic!("What?"),
        //     }
        // }
    }

    use tests_tool::*;
    
    #[test]
    /// ExecuteMsg::CreateTransaction 테스트 
    fn test_create_transaction_token_to_nft() {
        // buyer 가 부족한 자금을 보낸 경우
        let config = generate_contract_config();
        let (mut app, contract) = set_contract(config.clone());
        
        let msg = ExecuteMsg::CreateTransaction {
            seller: Addr::unchecked("seller").to_string(),
            desired_item: "nft".to_string(),
            nft_token_id: "2".to_string(),
        };
        let funds_sent = Coin::new(1u128, config.token_address.clone());
        let cosmos_msg = contract.call(msg, funds_sent).unwrap();
        let res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
        should_panic(res, ContractError::NotMatchExchangeRate);

        // buyer 가 잘못된 토큰을 보내는 경우
        let msg = ExecuteMsg::CreateTransaction {
            seller: Addr::unchecked("seller").to_string(),
            desired_item: "nft".to_string(),
            nft_token_id: "2".to_string(),
        };
        let funds_sent = Coin::new(10u128, "error".to_string());
        let cosmos_msg = contract.call(msg, funds_sent).unwrap();
        let res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
        should_panic(res, ContractError::UnauthorizedToken);


        // seller 는 info.sender 가 될 수 없음.
        let msg = ExecuteMsg::CreateTransaction {
            seller: Addr::unchecked(ADMIN).to_string(),
            desired_item: "nft".to_string(),
            nft_token_id: "2".to_string(),
        };
        let funds_sent = Coin::new(10u128, config.token_address.to_string());
        let cosmos_msg = contract.call(msg, funds_sent).unwrap();
        let res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
        should_panic(res, ContractError::BadTransaction);


        // 통과 확인
        let msg = ExecuteMsg::CreateTransaction {
            seller: Addr::unchecked("seller").to_string(),
            desired_item: "nft".to_string(),
            nft_token_id: "2".to_string(),
        };
        let funds_sent = Coin::new(10u128, config.token_address.clone());
        let cosmos_msg = contract.call(msg, funds_sent).unwrap();
        app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();
    }
    

    // #[test]
    /// ExecuteMsg::CreateTransaction 테스트 
    fn approve_transaction() {
        // transaction 이 존재하지 않은 상태서 approve 할 때.
        let config = generate_contract_config();
        let (mut app, contract) = set_contract(config.clone());

        let msg = ExecuteMsg::ApproveTransaction {
            buyer: ADMIN.to_string(),
        };
        let funds_sent = Coin::new(10u128, config.token_address.clone());
        let cosmos_msg = contract.call(msg, funds_sent).unwrap();
        let res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
        should_panic(res, ContractError::NoTransaction);

        // transaction 생성
        let msg = ExecuteMsg::CreateTransaction {
            seller: Addr::unchecked(USER).to_string(),
            desired_item: "nft".to_string(),
            nft_token_id: "2".to_string(),
        };
        let funds_sent = Coin::new(10u128, config.token_address.clone());
        let cosmos_msg = contract.call(msg, funds_sent).unwrap();
        app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();
        
        // 다른 사람이 transaction approve 할 경우 
        let msg = ExecuteMsg::ApproveTransaction {
            buyer: ADMIN.to_string(),
        };
        let funds_sent = Coin::new(10u128, config.token_address.clone());
        let cosmos_msg = contract.call(msg, funds_sent).unwrap();
        let res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
        should_panic(res, ContractError::NotDesignatedSeller);





        // let msg = ExecuteMsg::ApproveTransaction {
        //     buyer: ADMIN.to_string(),
        // };
        // let funds_sent = Coin::new(10u128, config.token_address.clone());
        // let cosmos_msg = contract.call(msg, funds_sent).unwrap();
        // let res = app.execute(Addr::unchecked(USER), cosmos_msg);
    }
}
