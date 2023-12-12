use openbrush::{
    traits::{AccountId, Balance, String},
};
use ink_prelude::vec::Vec;

#[openbrush::wrapper]
pub type LunesLottoRef = dyn LunesLotto;

#[openbrush::trait_definition]
pub trait LunesLotto {
    #[ink(message, payable)]
    fn play_lunes(&mut self, num: Vec<u64>) -> Result<(), ()>;
    #[ink(message, payable)]
    fn play_usdt(&mut self, num: Vec<u64>) -> Result<(), ()>;
    #[ink(message)]
    fn createRaffleLotto(&mut self,date_raffle:u64,price:Balance) -> Result<(), ()>;
    #[ink(message)]
    fn doRaffleLotto(&mut self) -> Result<(), ()>;
    #[ink(message)]
    fn myGames(&mut self, idTicket: u64, page: u64) -> Result<(), ()>;
    #[ink(message)]
    fn payment(&mut self, idTicket: u64) -> Result<(), ()>;
    #[ink(message)]
    fn allRaffle(&mut self, raffle_id: u64, page: u64) -> Result<(), ()>;
    #[ink(message)]
    fn winnerRaffle(&mut self, raffle_id: u64) -> Result<(), ()>;
}
