use openbrush::traits::{ AccountId, Balance, String };
use ink_prelude::vec::Vec;

pub type RaffleId = u64;
pub type TicketId = u64;
pub type NumRaffle = Vec<u64>;
pub type DateRaffle = u64;
pub type Status = bool;

pub type Owner = AccountId;
pub type Price = Balance;
pub type TotalAccumulated = Balance;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub rafflies: Vec<LunesLotto>,
    pub tickets: Vec<LunesTicket>,
}
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct LunesLotto{
    pub raffle_id: RaffleId,
    pub num_raffle: NumRaffle,
    pub date_raffle: u64,
    pub price: Price,
    pub total_accumulated: TotalAccumulated,
    pub status: Status,
}
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct LunesTicket{
    pub ticket_id: TicketId,
    pub raffle_id: RaffleId,
    pub game_raffle: NumRaffle,
    pub date_create: u64,
    pub owner: Owner,
}
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum LunesError {
    BadMintValue,
    CannotMintZeroTokens,
    CollectionIsFull,
    TooManyTokensToMint,
    WithdrawalFailed,
    BadNotFoundTokenId,
    BadNotOwnerTokenId,
    MaxPerMint,
}

impl LunesError {
    pub fn as_str(&self) -> String {
        match self {
            LunesError::BadMintValue => String::from("BadMintValue"),
            LunesError::CannotMintZeroTokens => String::from("CannotMintZeroTokens"),
            LunesError::CollectionIsFull => String::from("CollectionIsFull"),
            LunesError::TooManyTokensToMint => String::from("TooManyTokensToMint"),
            LunesError::WithdrawalFailed => String::from("WithdrawalFailed"),
            LunesError::BadNotFoundTokenId => String::from("BadNotFoundTokenId"),
            LunesError::BadNotOwnerTokenId => String::from("BadNotOwnerTokenId"),
            LunesError::MaxPerMint => String::from("MaxPerMint"),
        }
    }
}