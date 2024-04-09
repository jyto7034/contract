use cosmwasm_schema::write_api;
use std::env;
use main::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }

    env::set_var("RUST_BACKTRACE", "1");
}
