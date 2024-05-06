#![warn(clippy::arithmetic_side_effects)]

use crate::impls::lotto_lunes::data::{Data, InfoContract, ListNumRaffle, LunesError};
use ink_prelude::vec;
use ink_prelude::vec::Vec;
use openbrush::contracts::{
    ownable, ownable::only_owner, reentrancy_guard, reentrancy_guard::non_reentrant,
};
use openbrush::{
    contracts::traits::psp22::PSP22Error,
    modifiers,
    traits::{AccountId, Balance, Storage},
};

use super::data::LottoWin;
#[openbrush::trait_definition]
pub trait LottoLunesImpl:
    Storage<Data> + Storage<reentrancy_guard::Data> + Storage<ownable::Data> + Internal
{
    /// Payable in LUNES
    #[ink(message, payable)]
    #[modifiers(non_reentrant)]
    fn play_lunes(&mut self, num_rifle: Vec<ListNumRaffle>) -> Result<(), PSP22Error> {
        let caller = Self::env().caller();
        let value_pay = Self::env().transferred_value();
        self.check_amount(value_pay, num_rifle.clone())?;

        //Send tax_lunes
        let price_total = self.data::<Data>().price * (num_rifle.len() as u128);

        let tx_lunes = (value_pay - price_total) as Balance;

        let owner = self.data::<ownable::Data>().owner.get().unwrap().unwrap();
        Self::env()
            .transfer(owner, tx_lunes)
            .map_err(|_| PSP22Error::Custom(LunesError::WithdrawalFailed.as_str()))?;
        //create Ticket
        self.create_ticket(caller, num_rifle.clone())?;
        Ok(())
    }
    /// Create Raffle with Date, Price and Total Accumulated
    #[ink(message, payable)]
    fn create_raffle_lotto(&mut self, date_raffle: u64, price: Balance) -> Result<(), PSP22Error> {
        if self.data::<Data>().status {
            return Err(PSP22Error::Custom(LunesError::DrawNotFinish.as_str()));
        }
        let date_block = Self::env().block_timestamp();
        if date_block > date_raffle {
            return Err(PSP22Error::Custom(LunesError::NumInvalid.as_str()));
        }
        let caller = Self::env().caller();
        if !self.data::<Data>().account_do_lotto.iter().any(|f| f == &caller){
            return Err(PSP22Error::Custom(LunesError::NotAuthorized.as_str()));
        }
        self.data::<Data>().next_id += 1;
        let total_accumulated_next = Self::env().transferred_value();
        self.data::<Data>().total_accumulated += total_accumulated_next;
        self.data::<Data>().status = true;
        self.data::<Data>().price = price;
        Ok(())
    }
    /// Add Value in the Total Accumulated
    #[ink(message, payable)]
    fn add_accumulated(&mut self) -> Result<(), ()> {
        let accumulated = Self::env().transferred_value();
        self.data::<Data>().total_accumulated += accumulated;
        Ok(())
    }
    /// Random Lotto
    #[ink(message)]
    #[openbrush::modifiers(only_owner)]
    #[modifiers(non_reentrant)]
    fn random_lotto(&mut self) -> Result<Vec<u64>, PSP22Error> {
        let num_raffle = self.random();
        Ok(num_raffle)
    }
    /// Do Raffle in the date
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn do_raffle_lotto(&mut self) -> Result<Vec<u64>, PSP22Error> {
        if !self.data::<Data>().status {
            return Err(PSP22Error::Custom(LunesError::DrawNotStarted.as_str()));
        }
        let date_raffle = self.data::<Data>().date_raffle;
        let raffle_id = self.data::<Data>().next_id;
        let date_block = Self::env().block_timestamp();
        if date_block < date_raffle {
            return Err(PSP22Error::Custom(LunesError::DrawNotStarted.as_str()));
        }

        //todo call do_raffle
        self.data::<Data>().status = false;
        let num_raffle = self.random();
        let vec_num = vec![ListNumRaffle {
            is_payment: true,
            value_award: 0,
            num_1: num_raffle[0],
            num_2: num_raffle[1],
            num_3: num_raffle[2],
            num_4: num_raffle[3],
            num_5: num_raffle[4],
            num_6: num_raffle[5],
        }];
        self.data::<Data>().num_raffle.insert(raffle_id, &vec_num);
        Ok(num_raffle)
    }

    //Chage Tax Lunes
    #[ink(message)]
    #[openbrush::modifiers(only_owner)]
    #[modifiers(non_reentrant)]
    fn change_tx(&mut self, new_tx: u64) -> Result<(), PSP22Error> {
        self.data::<Data>().tx_lunes = new_tx;
        Ok(())
    }
    //Chage Tax Lunes
    #[ink(message)]
    fn info_contract(&mut self) -> Result<InfoContract, ()> {
        Ok(InfoContract {
            tx_lunes: self.data::<Data>().tx_lunes,
            date_raffle: self.data::<Data>().date_raffle,
            status: self.data::<Data>().status,
            raffle_id: self.data::<Data>().next_id,
            total_accumulated: self.data::<Data>().total_accumulated,
        })
    }
    //Get Numbers Deaws
    #[ink(message)]
    fn draw_num_raffle(&mut self, id_raffle: u64) -> Result<Vec<ListNumRaffle>, PSP22Error> {
        let number = self.data::<Data>().num_raffle.get(id_raffle);
        Ok(number.unwrap())
    }
    //Chage Tax Lunes
    #[ink(message)]
    fn my_play_per_raffle(&mut self, id_raffle: u64) -> Result<Vec<ListNumRaffle>, PSP22Error> {
        let caller = Self::env().caller();
        let games = self.data::<Data>().players.get((caller, id_raffle));
        Ok(games.unwrap())
    }

    #[ink(message)]
    fn payment(&mut self, id_raffle: u64) -> Result<(), PSP22Error> {
        let dranws = self.data::<Data>().winners.get(id_raffle);
        if dranws.is_some() {
            let num_raffle = self.data::<Data>().num_raffle.get(id_raffle).unwrap();
            let vet_num_raffle = vec![
                num_raffle[0].num_1,
                num_raffle[0].num_2,
                num_raffle[0].num_3,
                num_raffle[0].num_4,
                num_raffle[0].num_5,
                num_raffle[0].num_6,
            ];
            let caller = Self::env().caller();
            let games = self.data::<Data>().players.get((caller, id_raffle)).unwrap();
            let mut total_per_pay_2: Balance = 0;
            let mut total_per_pay_3: Balance = 0;
            let mut total_per_pay_4: Balance = 0;
            let mut total_per_pay_5: Balance = 0;
            let mut total_per_pay_6: Balance = 0;
            let mut vec_mut: Vec<ListNumRaffle> =  Vec::new();
            let item_d = dranws.unwrap();
            for g in games {
                let mut game_mut = g;
                if game_mut.is_payment == true {
                    continue;
                }
                let vet_num_player = vec![
                    game_mut.num_1, game_mut.num_2, game_mut.num_3, game_mut.num_4, game_mut.num_5, game_mut.num_6
                ];
                let matching_numbers = vet_num_player
                    .iter()
                    .filter(|&num| vet_num_raffle.clone().contains(num))
                    .count();
                match matching_numbers {
                    2 => {
                        total_per_pay_2 += item_d[0].value_award_2;
                        game_mut.value_award = item_d[0].value_award_2;
                        game_mut.is_payment = true;
                    }
                    3 => {
                        total_per_pay_3 += item_d[0].value_award_3;
                        game_mut.value_award = item_d[0].value_award_3;
                        game_mut.is_payment = true;
                    }
                    4 => {
                        total_per_pay_4 += item_d[0].value_award_4;
                        game_mut.value_award = item_d[0].value_award_4;
                        game_mut.is_payment = true;
                    }
                    5 => {
                        total_per_pay_5 += item_d[0].value_award_5;
                        game_mut.value_award = item_d[0].value_award_5;
                        game_mut.is_payment = true;
                    }
                    6 => {
                        total_per_pay_6 += item_d[0].value_award_6;
                        game_mut.value_award = item_d[0].value_award_6;
                        game_mut.is_payment = true;
                    }
                    _ => {}
                }
                vec_mut.push(game_mut);
            }
            let mut value_award_total: Balance = 0;
            if total_per_pay_2 != 0 {
                value_award_total += total_per_pay_2;
            }
            if total_per_pay_3 != 0 {
                value_award_total += total_per_pay_3;
            }
            if total_per_pay_4 != 0 {
                value_award_total += total_per_pay_4;
            }
            if total_per_pay_5 != 0 {
                value_award_total += total_per_pay_5;
            }
            if total_per_pay_6 != 0 {
                value_award_total += total_per_pay_6;
            }
            if value_award_total != 0 {
                self.data::<Data>().players.remove((caller, id_raffle));
                self.data::<Data>().players.insert((caller, id_raffle), &vec_mut);
                let owner_ = self.data::<ownable::Data>().owner.get().unwrap().unwrap();
                Self::env()
                    .transfer(owner_, value_award_total)
                    .map_err(|_| PSP22Error::Custom(LunesError::WithdrawalFailed.as_str()))?;
            }

            return Ok(());
        }

        Err(PSP22Error::Custom(LunesError::DrawNotStarted.as_str()))
    }
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn pre_payment_lotto(&mut self, lotto_win: LottoWin) -> Result<(), PSP22Error> {
        let caller = Self::env().caller();
        if !self.data::<Data>().account_do_lotto.iter().any(|f| f == &caller){
            return Err(PSP22Error::Custom(LunesError::NotAuthorized.as_str()));
        }
        if lotto_win.raffle_id == 0 {
            return Err(PSP22Error::Custom(LunesError::NotAuthorized.as_str()));
        }
        let total_fee = lotto_win.fee_lunes;
        let total_accumulated_next:Balance = lotto_win.total_accumulated_next;
        let id_ = lotto_win.raffle_id;
        let vec_win = vec![lotto_win];
        self.data::<Data>()
            .winners
            .insert(id_, &vec_win);
        let owner_ = self.data::<ownable::Data>().owner.get().unwrap().unwrap();
        Self::env()
            .transfer(owner_, total_fee)
            .map_err(|_| PSP22Error::Custom(LunesError::WithdrawalFailed.as_str()))?;
        self.data::<Data>().total_accumulated = total_accumulated_next;
        Ok(())
    }
    //add account lotto
    #[ink(message)]
    #[openbrush::modifiers(only_owner)]
    #[modifiers(non_reentrant)]
    fn add_account_lotto(&mut self, account: AccountId) -> Result<(), PSP22Error> {
        self.data::<Data>().account_do_lotto.push(account);
        Ok(())
    }
}
pub trait Internal: Storage<Data> {
    /// Check amount payable
    fn check_amount(
        &self,
        transferred_value: Balance,
        num_raffle: Vec<ListNumRaffle>,
    ) -> Result<(), PSP22Error> {
        if !self.data::<Data>().status {
            return Err(PSP22Error::Custom(LunesError::RaffleNotActive.as_str()));
        }
        let mut price_total = self.data::<Data>().price * (num_raffle.len() as u128);
        let tx_lunes = self.data::<Data>().tx_lunes;
        let tax_lunes: u128 = ((price_total * (tx_lunes as u128)) / 100) as u128;
        price_total += tax_lunes;
        if price_total <= transferred_value {
            for vet_num in num_raffle {
                let vec_ = [
                    vet_num.num_1,
                    vet_num.num_2,
                    vet_num.num_3,
                    vet_num.num_4,
                    vet_num.num_5,
                    vet_num.num_6,
                ];
                let mut duplicates = Vec::new();

                for i in 0..vec_.len() {
                    for j in i + 1..vec_.len() {
                        if vec_[i] == vec_[j] && !duplicates.contains(&vec_[i]) {
                            duplicates.push(vec_[i]);
                        }
                    }
                }

                if duplicates.len() > 0 {
                    return Err(PSP22Error::Custom(LunesError::NumRepeating.as_str()));
                }

                let is_exist_zero = vec_.iter().any(|x| x == &0);
                if is_exist_zero {
                    return Err(PSP22Error::Custom(LunesError::NumInvalid.as_str()));
                }

                let is_exist_num_invalid = vec_.iter().any(|x| x > &60);
                if is_exist_num_invalid {
                    return Err(PSP22Error::Custom(LunesError::NumSuper60.as_str()));
                }
            }
            return Ok(());
        }
        Err(PSP22Error::Custom(LunesError::BadMintValue.as_str()))
    }

    fn create_ticket(
        &mut self,
        to: AccountId,
        mut num_raffle: Vec<ListNumRaffle>,
    ) -> Result<(), PSP22Error> {
        let lotto_id = self.data::<Data>().next_id;
        let games = self.data::<Data>().players.get((to, lotto_id));
        if games.is_some() {
            num_raffle.extend(games.unwrap());
            self.data::<Data>().players.remove((to, lotto_id));
        }
        self.data::<Data>()
            .players
            .insert((to, lotto_id), &num_raffle);

        Ok(())
    }
    /// Generates a seed based on the list of players and the block number and timestamp
    fn seed(&self, seed: u64) -> u64 {
        let timestamp = Self::env().block_timestamp() + seed;
        let block_number = Self::env().block_number() as u64 + seed;

        timestamp ^ block_number
    }

    fn random(&self) -> Vec<u64> {
        let mut unique_numbers = Vec::new();
        let mut increment: u64 = 0;
        while unique_numbers.len() < 6 {
            // Generate the seed using the existing seed function
            let mut x = self.seed(Self::env().block_timestamp() + increment);

            // Manipulate the seed to get a pseudo-random result
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;

            // Map the random number to the range [1, 60]
            let random_number = ((x % 60) + 1) as u64;
            increment += random_number;
            // Ensure the generated number is unique
            if !unique_numbers.contains(&random_number) {
                unique_numbers.push(random_number);
            }
        }

        unique_numbers
    }
}
