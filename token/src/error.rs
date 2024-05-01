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

    #[error("SoldOut")]
    AlreadyInTransaction,

    #[error("Contract is lock")]
    Lock,

    #[error("Bad funds")]
    BadFunds,

    #[error("Bad funds")]
    NoTransaction,

    #[error("Too high block")]
    HighBlock,

    #[error("Bad Transaction")]
    BadTransaction,

    #[error("Reserved NFT")]
    ReservedNFT,

    #[error("The transaction is already in progress..")]
    TransactionAlreadyProgress,

    #[error("Unknown Error.")]
    UnknownError,

    #[error("The nft in the contract is insufficient.")]
    NotEnoughContractNFT,

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

    #[error("Unauthorized Nft")]
    UnauthorizedNft,

    #[error("Not enough funds")]
    NoFunds,

    #[error("Escrow expired")]
    Expired,

    #[error("Escrow not expired")]
    NotExpired,

    #[error("Not received funds. attach --amount")]
    NotReceivedFunds,

    #[error("Exchange rates do not match")]
    NotMatchExchangeRate,

    #[error("Bad Transaction Info.")]
    BadTransactionInfo,

    #[error("Product Token Parsing Failed")]
    ProductTokenPasingErr,

    #[error("You don't own it.")]
    DoesNotOwnNFT,

    #[error("Not Designated Seller")]
    NotDesignatedSeller,
}