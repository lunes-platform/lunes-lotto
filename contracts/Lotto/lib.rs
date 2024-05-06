#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[openbrush::implementation(Ownable)]
#[openbrush::contract]
pub mod lotto_lunes{
    use openbrush::{
        contracts::{
            ownable,           
            reentrancy_guard,
        },
        traits::Storage,
    };
    use lotto_lunes_pkg::impls::lotto_lunes::{lotto_lunes::*, data };
    use ink::storage::Mapping;
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct LottoLunesContract {
        #[storage_field]
        payable_lotto: data::Data,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl lotto_lunes_pkg::impls::lotto_lunes::lotto_lunes::Internal for LottoLunesContract {}
    impl LottoLunesImpl for LottoLunesContract {}

    impl LottoLunesContract {
        #[ink(constructor)]
        pub fn new(date_raffle: u64,price: Balance, tx_fee: u64) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            ownable::InternalImpl::_init_with_owner(&mut instance, caller);

            let mut instance = Self::default();
            instance.payable_lotto.next_id = 1;
            instance.payable_lotto.tx_lunes = tx_fee;
            instance.payable_lotto.price = price;
            instance.payable_lotto.status = true;
            instance.payable_lotto.date_raffle = date_raffle;
            instance.payable_lotto.total_accumulated = 0;
            instance.payable_lotto.num_raffle = Mapping::default();
            instance.payable_lotto.players = Mapping::default();
            instance.payable_lotto.winners = Mapping::default();
            instance.payable_lotto.account_do_lotto = Default::default();
            instance
        }
    }    
  
}