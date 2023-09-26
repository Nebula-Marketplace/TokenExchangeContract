use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Not found")]
    NotFound {},

    #[error("Invalid amount")]
    InvalidAmount {},
    
    #[error("Unknown error")]
    UnknownError {},
}