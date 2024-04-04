use cosmwasm_std::StdError;
use thiserror::Error;
use cw_utils::Expiration;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    
    #[error("The nft in the contract is insufficient.")]
    NotEnoughContractNFT,

    #[error("The token in the contract is insufficient.")]
    NotEnoughContractTokens,

    #[error("Balance Query Failed")]
    BalanceQueryFailed,

    #[error("Unauthorized Addr")]
    UnauthorizedAddr,

    #[error("Unauthorized token")]
    UnauthorizedToken,

    #[error("Unauthorized Nft")]
    UnauthorizedNft,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not enough funds")]
    NoFunds,

    #[error("Escrow expired (expiration: {expiration:?})")]
    Expired { expiration: Expiration },

    #[error("Not received funds. attach --amount")]
    NotReceivedFunds,
    
    #[error("Exchange rates do not match")]
    NotMatchExchangeRate,

    
    #[error("Product Token Parsing Failed")]
    ProductTokenPasingErr,
    
    #[error("Does not own the corresponding nft. ")]
    DoesNotOwnNFT,

    
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
