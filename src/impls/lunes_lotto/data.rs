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
pub type ValueAward = Balance;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub next_id: RaffleId,
    pub rafflies: Vec<LunesLotto>,
    pub tickets: Vec<LunesTicket>,
    pub winners: Vec<LunesTicket>,
}
#[derive(Debug, PartialEq,Clone, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct LunesLotto{
    pub raffle_id: RaffleId,
    pub num_raffle: NumRaffle,
    pub date_raffle: u64,
    pub price: Price,
    pub total_accumulated: TotalAccumulated,
    pub status: Status,
}
#[derive(Debug, PartialEq,Clone, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct LunesTicket{
    pub raffle_id: RaffleId,
    pub game_raffle: NumRaffle,
    pub date_create: u64,
    pub value_award: ValueAward,
    pub owner: Owner,
    pub status: Status,
    
}
#[derive(Debug, PartialEq,Clone, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ListNumRaffle{
    pub num_1: u64,
    pub num_2: u64,
    pub num_3: u64,
    pub num_4: u64,
    pub num_5: u64,
    pub num_6: u64,
}
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum LunesError {
    BadMintValue,
    DrawNotStarted,
    WithdrawalFailed
}

impl LunesError {
    pub fn as_str(&self) -> String {
        match self {
            LunesError::BadMintValue => String::from("BadMintValue"),
            LunesError::DrawNotStarted => String::from("DrawNotStarted"),
            LunesError::WithdrawalFailed => String::from("WithdrawalFailed"),
        }
    }
}