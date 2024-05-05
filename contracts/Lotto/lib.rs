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
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            ownable::InternalImpl::_init_with_owner(&mut instance, caller);

            let mut instance = Self::default();
            instance.payable_lotto.next_id = 1;
            instance.payable_lotto.next_ticket_id =1;
            instance.payable_lotto.tx_lunes = 17;
            instance.payable_lotto.total_accumulated_next = 0;
            instance.payable_lotto.rafflies = Default::default();
            instance.payable_lotto.tickets = Default::default();
            instance.payable_lotto.winners = Default::default();
            instance
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[ink::test]
        fn random_lotto() {
            let mut contract = LottoLunesContract::new();           
            assert!(contract.random_lotto().is_ok());    
        }
        #[ink::test]
        fn create_automatic_lotto() {
            let mut contract = LottoLunesContract::new();
            assert!(contract.random_lotto().is_ok());
        }
    }
  
}