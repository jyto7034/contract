use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("NoInitalizeDenom")]
    NoInitalizeDenom,

    #[error("Contract is lock")]
    Lock,

    #[error("Bad funds")]
    BadFunds,

    #[error("Bad funds")]
    NoTransaction,

    #[error("Bad funds")]
    Denied,

    #[error("Bad Transaction")]
    BadTransaction,

    #[error("The transaction is already in progress..")]
    TransactionAlreadyProgress,

    #[error("Unknown Error.")]
    UnknownError,

    #[error("Balance Query Failed")]
    BalanceQueryFailed,

    #[error("Balance insufficient")]
    Balanceinsufficient,

    #[error("Unauthorized Addr")]
    UnauthorizedAddr,

    #[error("Unauthorized token")]
    UnauthorizedToken,

    #[error("Does not have Token")]
    DoesNotHaveToken,

    #[error("Not received funds. attach --amount")]
    NotReceivedFunds,

    #[error("Exchange rates do not match")]
    NotMatchExchangeRate,

    #[error("You don't own it.")]
    DoesNotOwnNFT,
}