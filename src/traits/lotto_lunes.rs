use ink::primitives::AccountId;
use openbrush::traits::Balance;
use ink_prelude::vec::Vec;

#[openbrush::wrapper]
pub type LottoLunesRef = dyn LottoLunes;

#[openbrush::trait_definition]
pub trait LottoLunes {
    #[ink(message, payable)]
    fn play_lunes(&mut self, num: Vec<u64>) -> Result<(), ()>;    
    #[ink(message)]
    fn create_raffle_lotto(&mut self,date_raffle:u64,price:Balance) -> Result<(), ()>;
    #[ink(message)]
    fn do_raffle_lotto(&mut self) -> Result<(), ()>;
    #[ink(message)]
    fn my_games(&mut self, page: u64) -> Result<(), ()>;
    #[ink(message)]
    fn payment(&mut self, id_ticket: u64) -> Result<(), ()>;
    #[ink(message)]
    fn all_raffle(&mut self, raffle_id: u64, page: u64) -> Result<(), ()>;
    #[ink(message)]
    fn winner_raffle(&mut self, raffle_id: u64) -> Result<(), ()>;
    #[ink(message)]
    fn random_lotto(&mut self) -> Result<(), ()>;
    #[ink(message)]
    fn payment_expired(&mut self, ticket_id: u64) -> Result<(), ()>;
    #[ink(message)]
    fn transfer_ticket_to(&mut self, to: AccountId, ticket_id: u64) -> Result<(), ()>; 
    #[ink(message)]
    fn all_raffle_page(&mut self, raffle_id: u64, page: u64) -> Result<(), ()>;
    #[ink(message)]
    fn my_games_page(&mut self, page: u64) -> Result<(), ()>;
    #[ink(message)]
    fn info_contract(&mut self) -> Result<(), ()>;
    #[ink(message)]
    fn change_tx(&mut self, new_tx: u64) -> Result<(), ()>;
}
