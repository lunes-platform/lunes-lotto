use openbrush::traits::{ AccountId, Balance, String };
use ink_prelude::vec::Vec;
use ink::storage::Mapping;
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
    pub account_do_lotto: Vec<AccountId>,
    pub next_id: RaffleId,
    pub date_raffle: u64,
    pub price: Price,
    pub status: Status,
    pub players: Mapping<(Owner,RaffleId),Vec<ListNumRaffle>>,
    pub num_raffle: Mapping<RaffleId,Vec<ListNumRaffle>>,
    pub winners: Mapping<RaffleId,Vec<LottoWin>>,
    pub tx_lunes: TxLunes,
    pub total_accumulated: TotalAccumulated,
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
    pub is_payment: Status,
    pub value_award: ValueAward,
}
#[derive(Debug, PartialEq, Clone, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct InfoContract {
    pub tx_lunes: u64,
    pub date_raffle: u64,
    pub status: Status,
    pub raffle_id: RaffleId,
    pub total_accumulated: TotalAccumulated,
}
#[derive(Debug, PartialEq,Clone, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct LottoWin{
    pub date_create: u64,
    pub raffle_id: RaffleId,
    pub value_award_2: ValueAward,
    pub quantity_2: u64,
    pub value_award_3: ValueAward,
    pub quantity_3: u64,
    pub value_award_4: ValueAward,
    pub quantity_4: u64,
    pub value_award_5: ValueAward,
    pub quantity_5: u64,
    pub value_award_6: ValueAward,
    pub quantity_6: u64,
    pub fee_lunes: ValueAward,
    pub total_accumulated: ValueAward,
    pub total_accumulated_next: ValueAward,
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
    DrawNotFinish,
    NotAuthorized,
}

impl LunesError {
    pub fn as_str(&self) -> String {
        match self {
            LunesError::BadMintValue => String::from("BadMintValue"),
            LunesError::DrawNotStarted => String::from("DrawNotStarted"),
            LunesError::DrawNotFinish => String::from("DrawNotFinish"),
            LunesError::WithdrawalFailed => String::from("WithdrawalFailed"),
            LunesError::NumRepeating => String::from("NumRepeating"),
            LunesError::NumInvalid => String::from("NumInvalid"),
            LunesError::NumSuper60 => String::from("NumSuper60"),
            LunesError::InvalidPage => String::from("InvalidPage"),
            LunesError::BackRaffleNotFound => String::from("BackRaffleNotFound"),
            LunesError::RaffleNotActive => String::from("RaffleNotActive"),
            LunesError::PaymentExpired => String::from("PaymentExpired"),
            LunesError::NotAuthorized => String::from("NotAuthorized"),
            
        }
    }
}