use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Less than zero")]
    ZeorError {},

    #[error("Wrong Number")]
    WrongNumber {},

    
    #[error("Not enough funds")]
    Notenough{},

    
    #[error("Mint is ended")]
    MintEnded{},

    #[error("You can not mint anymore")]
    MintExceeded{},


    
    #[error("You are not white user")]
    WrongWhiteUser {},
}
