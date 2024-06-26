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
pub type Hits = u64;
pub type TxLunes = u64;
#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub next_id: RaffleId,
    pub next_ticket_id: RaffleId,
    pub rafflies: Vec<LottoLunes>,
    pub tickets: Vec<LunesTicket>,
    pub winners: Vec<LunesTicket>,
    pub tx_lunes: TxLunes
}
#[derive(Debug, PartialEq,Clone, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct LottoLunes{
    pub date_create: u64,
    pub raffle_id: RaffleId,
    pub num_raffle: NumRaffle,
    pub date_raffle: u64,
    pub price: Price,
    pub total_accumulated: TotalAccumulated,
    pub total_accumulated_next: TotalAccumulated,
    pub status: Status,
    pub status_done: Status,
}
#[derive(Debug, PartialEq,Clone, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct LunesTicket{
    pub raffle_id: RaffleId,
    pub ticket_id: TicketId,
    pub game_raffle: NumRaffle,
    pub date_create: u64,
    pub value_award: ValueAward,
    pub hits: Hits,
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

#[derive(Debug, PartialEq, Clone, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PageListRaffle {
    pub count: u64,
    pub page: u64,
    pub loto_lunes: Vec<LottoLunes>,
    
}
#[derive(Debug, PartialEq,Clone, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PageListTicket {
    pub count: u64,
    pub page: u64,
    pub tickets: Vec<LunesTicket>,
    
}
#[derive(Debug, PartialEq, Clone, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct InfoContract {
    pub tx_lunes: u64,
    pub count_lotto: u64,
    pub count_tickets: u64,
}
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum LunesError {
    BadMintValue,
    DrawNotStarted,
    WithdrawalFailed,
    NumRepeating,
    NumInvalid,
    NumSuper60,
    InvalidPage,
    BackRaffleNotFound,
    RaffleNotActive,
    PaymentExpired,
}

impl LunesError {
    pub fn as_str(&self) -> String {
        match self {
            LunesError::BadMintValue => String::from("BadMintValue"),
            LunesError::DrawNotStarted => String::from("DrawNotStarted"),
            LunesError::WithdrawalFailed => String::from("WithdrawalFailed"),
            LunesError::NumRepeating => String::from("NumRepeating"),
            LunesError::NumInvalid => String::from("NumInvalid"),
            LunesError::NumSuper60 => String::from("NumSuper60"),
            LunesError::InvalidPage => String::from("InvalidPage"),
            LunesError::BackRaffleNotFound => String::from("BackRaffleNotFound"),
            LunesError::RaffleNotActive => String::from("RaffleNotActive"),
            LunesError::PaymentExpired => String::from("PaymentExpired"),
            
        }
    }
}