use cosmwasm_schema::write_api;
use main::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use std::env;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }

    env::set_var("RUST_BACKTRACE", "1");
}
