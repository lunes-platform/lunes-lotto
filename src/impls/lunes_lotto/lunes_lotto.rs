
use crate::impls::lunes_lotto::data::{ Data, LunesLotto, LunesTicket, LunesError };
use openbrush::{traits::{ AccountId, Balance, Storage, String }, contracts::ownable::OwnableError};
use ink_prelude::vec::Vec;
use ink_prelude::string::ToString;
use openbrush::contracts::{
    ownable,
    ownable::only_owner,
    reentrancy_guard,
    reentrancy_guard::non_reentrant,
};
#[openbrush::trait_definition]
pub trait LunesLottoImpl: 
    Storage<Data>  
    + Storage<reentrancy_guard::Data>
    + Storage<ownable::Data>
    + Internal {
    /// Payable in LUNES    
    #[ink(message, payable)]
    fn play_lunes(&mut self, num: Vec<u64>) -> Result<(), LunesError>{
        Ok(())
    }
    /// Payable in USDT
    #[ink(message, payable)]
    fn play_usdt(&mut self, num: Vec<u64>) -> Result<(), LunesError>{
        Ok(())
    }
    /// Create Raffle with Date, Price and Total Accumulated
    #[ink(message, payable)]
    #[openbrush::modifiers(only_owner)]
    fn createRaffleLotto(&mut self,date_raffle:u64,price:Balance) -> Result<(), OwnableError>{
        Ok(())
    }
    /// Do Raffle in the date
    #[ink(message)]
    fn doRaffleLotto(&mut self) -> Result<(), LunesError>{
        Ok(())
    }
    /// List all Games the user has played
    #[ink(message)]
    fn myGames(&mut self, idTicket: u64, page: u64) -> Result<(), LunesError>{
        Ok(())
    }
    /// Payable in LUNES to ticket
    #[ink(message)]
    fn payment(&mut self, idTicket: u64) -> Result<(), LunesError>{
        Ok(())
    }
    /// List all Raffles
    #[ink(message)]
    fn allRaffle(&mut self, raffle_id: u64, page: u64) -> Result<(), LunesError>{
        Ok(())
    }
    /// Get Winner Raffle by id
    #[ink(message)]
    fn winnerRaffle(&mut self, raffle_id: u64) -> Result<(), LunesError>{
        Ok(())
    }

}
pub trait Internal: Storage<Data> {
    
}