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
    #[ink(message, payable)]
    fn my_play(&mut self, id_raffle: u64) -> Result<(), ()>;    
    #[ink(message)]
    fn info_contract(&mut self) -> Result<(), ()>;
    #[ink(message)]
    fn draw_num_raffle(&mut self,id_raffle: u64) -> Result<(), ()>;
    #[ink(message)]
    fn change_tx(&mut self, new_tx: u64) -> Result<(), ()>;
    #[ink(message)]
    fn payment(&mut self, id_raffle: u64) -> Result<(), ()>;
}
