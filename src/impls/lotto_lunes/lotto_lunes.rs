#![warn(clippy::arithmetic_side_effects)]
use super::data::{InfoContract, ListNumRaffle, PageListRaffle, PageListTicket, TicketId};
use crate::impls::lotto_lunes::data::{Data, LottoLunes, LunesError, LunesTicket};
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
#[openbrush::trait_definition]
pub trait LottoLunesImpl:
    Storage<Data> + Storage<reentrancy_guard::Data> + Storage<ownable::Data> + Internal
{
    /// Payable in LUNES
    #[ink(message, payable)]
    #[modifiers(non_reentrant)]
    fn play_lunes(&mut self, num_rifle: Vec<ListNumRaffle>) -> Result<(), PSP22Error> {
        let caller = Self::env().caller();
        let date_block = Self::env().block_timestamp();
        let raffle_id = self.check_amount(Self::env().transferred_value(), num_rifle.clone())?;
        let mut value_pay = Self::env().transferred_value();
        let tx_lunes = self.data::<Data>().tx_lunes;
        let tax_lunes: u128 = (value_pay * (tx_lunes as u128)) / 100;
        value_pay = value_pay - tax_lunes;
        //update Raffle
        let index = self
            .data::<Data>()
            .rafflies
            .iter()
            .position(|ix| ix.raffle_id == raffle_id)
            .unwrap();
        self.data::<Data>().rafflies[index].total_accumulated += value_pay;
        //Send tax_lunes
        let owner = self.data::<ownable::Data>().owner.get().unwrap().unwrap();
        Self::env()
            .transfer(owner, tax_lunes)
            .map_err(|_| PSP22Error::Custom(LunesError::WithdrawalFailed.as_str()))?;
        //create Ticket
        self.create_ticket(raffle_id, caller, num_rifle.clone(), date_block)?;
        Ok(())
    }
    /// Create Raffle with Date, Price and Total Accumulated
    #[ink(message, payable)]
    #[openbrush::modifiers(only_owner)]
    fn create_raffle_lotto(&mut self, date_raffle: u64, price: Balance) -> Result<(), PSP22Error> {
        let id = self.data::<Data>().next_id;
        //Verify Raffle active
        let refflet_active = self
            .data::<Data>()
            .rafflies
            .iter()
            .any(|raffle| raffle.status == true);

        let mut total_accumulated_next = Self::env().transferred_value();
        let back_reffer_index = self
            .data::<Data>()
            .rafflies
            .iter()
            .position(|raffle| raffle.raffle_id == id - 1);
        if back_reffer_index.is_some() {
            total_accumulated_next +=
                self.data::<Data>().rafflies[back_reffer_index.unwrap()].total_accumulated_next;
        }
        let date_block = Self::env().block_timestamp();
        self.data::<Data>().rafflies.push(LottoLunes {
            raffle_id: id,
            num_raffle: Vec::new(),
            date_raffle,
            price,
            total_accumulated: total_accumulated_next,
            status: !refflet_active,
            total_accumulated_next: 0,
            status_done: false,
            date_create: date_block,
        });
        self.data::<Data>().next_id += 1;

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
    /// Create Automatic Raffle
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn create_automatic_lotto(&mut self, back_raffle_id: u64) -> Result<(), PSP22Error> {
        let next_id = self.data::<Data>().next_id;
        //Verify Raffle active
        if self
            .data::<Data>()
            .rafflies
            .iter()
            .any(|raffle| raffle.status == true)
        {
            return Err(PSP22Error::Custom(LunesError::DrawNotStarted.as_str()));
        } else {
            let new_reffer_index = self
                .data::<Data>()
                .rafflies
                .iter()
                .position(|raffle| raffle.raffle_id == back_raffle_id +1);
            let back_reffer_index = self
                .data::<Data>()
                .rafflies
                .iter()
                .position(|raffle| raffle.raffle_id == back_raffle_id && raffle.status_done);

            if back_reffer_index.is_none() {
                return Err(PSP22Error::Custom(LunesError::BackRaffleNotFound.as_str()));
            }
            if new_reffer_index.is_none() {                
                let date_block = Self::env().block_timestamp();
                let price_ticket = self.data::<Data>().rafflies[back_reffer_index.unwrap()].price;
                let value_award_next =
                    self.data::<Data>().rafflies[back_reffer_index.unwrap()].total_accumulated_next;
                self.data::<Data>().rafflies.push(LottoLunes {
                    raffle_id: next_id,
                    num_raffle: Vec::new(),
                    date_raffle: date_block + 259343000,
                    price: price_ticket,
                    total_accumulated: value_award_next,
                    total_accumulated_next: 0,
                    status: true,
                    status_done: false,
                    date_create: date_block,
                });
                self.data::<Data>().next_id += 1;
            }else{
                self.data::<Data>().rafflies[new_reffer_index.unwrap()].status = true;
            }
        }

        Ok(())
    }
    /// Do Raffle in the date
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn do_raffle_lotto(&mut self) -> Result<Vec<u64>, PSP22Error> {
        let index = self
            .data::<Data>()
            .rafflies
            .iter()
            .position(|raffle| raffle.status == true);
        if index.is_none() {
            return Err(PSP22Error::Custom(LunesError::DrawNotStarted.as_str()));
        }
        let date_raffle = self.data::<Data>().rafflies[index.unwrap()].date_raffle;
        let raffle_id = self.data::<Data>().rafflies[index.unwrap()].raffle_id;
        let total_accumulated = self.data::<Data>().rafflies[index.unwrap()].total_accumulated;
        let value_award_2 = (total_accumulated * 2) / 100;
        let value_award_3 = (total_accumulated * 5) / 100;
        let value_award_4 = (total_accumulated * 10) / 100;
        let value_award_5 = (total_accumulated * 25) / 100;
        let value_award_6 =
            total_accumulated - (value_award_2 + value_award_3 + value_award_4 + value_award_5);

        let date_block = Self::env().block_timestamp();
        if date_block < date_raffle {
            return Err(PSP22Error::Custom(LunesError::DrawNotStarted.as_str()));
        }
        //todo call do_raffle
        self.data::<Data>().rafflies[index.unwrap()].status = false;
        let num_raffle = self.random();
        self.data::<Data>().rafflies[index.unwrap()].num_raffle = num_raffle.clone();
        //Done
        self.data::<Data>().rafflies[index.unwrap()].status_done = true;

        //find Winner
        let mut winner: Vec<LunesTicket> = Vec::new();

        let mut tickets = self
            .data::<Data>()
            .tickets
            .iter()
            .filter(|ticket| ticket.raffle_id == raffle_id)
            .collect::<Vec<&LunesTicket>>();
        let mut total_per_pay_2 = 0;
        let mut total_per_pay_3 = 0;
        let mut total_per_pay_4 = 0;
        let mut total_per_pay_5 = 0;
        let mut total_per_pay_6 = 0;
        for w in tickets.iter_mut() {
            let game_raffle = w.game_raffle.clone();
            let matching_numbers = game_raffle
                .iter()
                .filter(|&num| num_raffle.clone().contains(num))
                .count();
            let mut wi = w.clone();
            wi.status = true;
            wi.date_create = Self::env().block_timestamp();
            match matching_numbers {
                2 => {
                    total_per_pay_2 += 1;
                    wi.hits = 2 as u64;
                    wi.value_award = value_award_2;
                    winner.push(wi.clone());
                }
                3 => {
                    total_per_pay_3 += 1;
                    wi.hits = 3;
                    wi.value_award = value_award_3;
                    winner.push(wi.clone());
                }
                4 => {
                    total_per_pay_4 += 1;
                    wi.hits = 4;
                    wi.value_award = value_award_4;
                    winner.push(wi.clone());
                }
                5 => {
                    total_per_pay_5 += 1;
                    wi.hits = 5;
                    wi.value_award = value_award_5;
                    winner.push(wi.clone());
                }
                6 => {
                    total_per_pay_6 += 1;
                    wi.hits = 6;
                    wi.value_award = value_award_6;
                    winner.push(wi.clone());
                }
                _ => {}
            }
        }
        self.data::<Data>().winners.extend(winner.clone());
        //Distribuition of LUNES
        let mut value_award_next = 0;
        if total_per_pay_2 == 0 {
            value_award_next += value_award_2;
        }
        if total_per_pay_3 == 0 {
            value_award_next += value_award_3;
        }
        if total_per_pay_4 == 0 {
            value_award_next += value_award_4;
        }
        if total_per_pay_5 == 0 {
            value_award_next += value_award_5;
        }
        if total_per_pay_6 == 0 {
            value_award_next += value_award_6;
        }
        //if accumulated then 50% Lunes
        if value_award_next != 0 {
            let payment_lunes = (value_award_next * 50) / 100;
            value_award_next -= payment_lunes;
            let owner_ = self.data::<ownable::Data>().owner.get().unwrap().unwrap();
            Self::env()
                .transfer(owner_, payment_lunes)
                .map_err(|_| PSP22Error::Custom(LunesError::WithdrawalFailed.as_str()))?;
        }

        self.data::<Data>().rafflies[index.unwrap()].total_accumulated_next = value_award_next;

        Ok(num_raffle)
    }
    /// List all Games the user has played
    #[ink(message)]
    fn my_games(&mut self, page: u64) -> Result<Vec<LunesTicket>, PSP22Error> {
        if page == 0 {
            return Err(PSP22Error::Custom(LunesError::InvalidPage.as_str()));
        }
        let games = self
            .data::<Data>()
            .tickets
            .iter()
            .filter(|ticket| ticket.owner == Self::env().caller())
            .cloned()
            .rev()
            .skip(((page - 1) * (100 as u64)).try_into().unwrap())
            .take(100)
            .collect();

        Ok(games)
    }
    /// Payable in LUNES to ticket
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn payment(&mut self, ticket_id: TicketId) -> Result<(), PSP22Error> {
        if let Some(_) = self.data::<Data>().winners.iter().find(|winner| {
            winner.owner == Self::env().caller()
                && winner.status == true
                && winner.ticket_id == ticket_id
        }) {
            let index = self
                .data::<Data>()
                .winners
                .iter()
                .position(|winner| {
                    winner.owner == Self::env().caller()
                        && winner.status == true
                        && winner.ticket_id == ticket_id
                })
                .unwrap();
            //verify date received payment at 90 days
            let now = Self::env().block_timestamp();
            if now - self.data::<Data>().winners[index].date_create > 90 * 24 * 60 * 60 {
                return Err(PSP22Error::Custom(LunesError::PaymentExpired.as_str()));
            }
            self.data::<Data>().winners[index].status = false;

            let owner_ = self.data::<ownable::Data>().owner.get().unwrap().unwrap();
            Self::env()
                .transfer(owner_, self.data::<Data>().winners[index].value_award)
                .map_err(|_| PSP22Error::Custom(LunesError::WithdrawalFailed.as_str()))?;
            return Ok(());
        }
        Err(PSP22Error::Custom(LunesError::WithdrawalFailed.as_str()))
    }

    /// Payable in LUNES to ticket expired for 90 days
    #[ink(message)]
    #[openbrush::modifiers(only_owner)]
    #[modifiers(non_reentrant)]
    fn payment_expired(&mut self, ticket_id: TicketId) -> Result<(), PSP22Error> {
        if let Some(_) = self
            .data::<Data>()
            .winners
            .iter()
            .find(|winner| winner.status == true && winner.ticket_id == ticket_id)
        {
            let index = self
                .data::<Data>()
                .winners
                .iter()
                .position(|winner| winner.status == true && winner.ticket_id == ticket_id)
                .unwrap();
            //verify date received payment at 90 days
            let now = Self::env().block_timestamp();
            if now - self.data::<Data>().winners[index].date_create > (90 * 24 * 60 * 60) as u64 {
                let owner = self.data::<ownable::Data>().owner.get().unwrap().unwrap();
                self.data::<Data>().winners[index].status = false;
                Self::env()
                    .transfer(owner, self.data::<Data>().winners[index].value_award)
                    .unwrap();
                return Ok(());
            }
        }
        Err(PSP22Error::Custom(LunesError::WithdrawalFailed.as_str()))
    }
    /// List all Raffles
    #[ink(message)]
    fn all_raffle(&mut self, raffle_id: u64, page: u64) -> Result<Vec<LottoLunes>, PSP22Error> {
        if page == 0 {
            return Err(PSP22Error::Custom(LunesError::InvalidPage.as_str()));
        }
        let mut _games: Vec<LottoLunes> = Vec::new();
        if raffle_id == 0 {
            _games = self
                .data::<Data>()
                .rafflies
                .iter()
                .cloned()
                .rev()
                .skip(((page - (1 as u64)) * (100 as u64)).try_into().unwrap())
                .take(100)
                .collect();
        } else {
            _games = self
                .data::<Data>()
                .rafflies
                .iter()
                .filter(|riff| riff.raffle_id == raffle_id)
                .cloned()
                .rev()
                .skip(((page - (1 as u64)) * (100 as u64)).try_into().unwrap())
                .take(100)
                .collect();
        }
        Ok(_games)
    }

    /// Get Winner Raffle by id
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn winner_raffle(&mut self, raffle_id: u64) -> Result<Vec<LunesTicket>, PSP22Error> {
        let winners = self
            .data::<Data>()
            .winners
            .iter()
            .filter(|winner| winner.raffle_id == raffle_id)
            .cloned()
            .collect();
        return Ok(winners);
    }
    /// Transfer ticket to
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn transfer_ticket_to(&mut self, to: AccountId, ticket_id: TicketId) -> Result<(), PSP22Error> {
        let caller = Self::env().caller();
        if self
            .data::<Data>()
            .tickets
            .iter()
            .any(|ticket| ticket.owner == caller && ticket.ticket_id == ticket_id)
        {
            let index = self
                .data::<Data>()
                .tickets
                .iter()
                .position(|ticket| ticket.owner == caller && ticket.ticket_id == ticket_id)
                .unwrap();
            //verify raffel active
            let raffler_id = self.data::<Data>().tickets[index].raffle_id;
            let index_raffle = self
                .data::<Data>()
                .rafflies
                .iter()
                .position(|raffle| raffle.status && raffle.raffle_id == raffler_id);
            if index_raffle.is_none() {
                return Err(PSP22Error::Custom(LunesError::RaffleNotActive.as_str()));
            }
            //Do transfer
            self.data::<Data>().tickets[index].owner = to;
            return Ok(());
        }
        Err(PSP22Error::Custom(LunesError::WithdrawalFailed.as_str()))
    }
    /// Return All Raffle with count page
    #[ink(message)]
    fn all_raffle_page(&mut self, page: u64, done: bool) -> Result<PageListRaffle, PSP22Error> {
        if page == 0 {
            return Err(PSP22Error::Custom(LunesError::InvalidPage.as_str()));
        }
        let mut _games: Vec<LottoLunes> = Vec::new();
        _games = self
            .data::<Data>()
            .rafflies
            .iter()
            .filter(|riff| riff.status_done == done)
            .cloned()
            .rev()
            .skip(((page - (1 as u64)) * (100 as u64)).try_into().unwrap())
            .take(100)
            .collect();
        let count = self.data::<Data>().rafflies.iter().count() as u64;

        Ok(PageListRaffle {
            count: count.clone(),
            page,
            loto_lunes: _games,
        })
    }
    /// List all Game page
    #[ink(message)]
    fn my_games_page(&mut self, page: u64, done: bool) -> Result<PageListTicket, PSP22Error> {
        if page == 0 {
            return Err(PSP22Error::Custom(LunesError::InvalidPage.as_str()));
        }
        let tickets: Vec<LunesTicket> = self
            .data::<Data>()
            .tickets
            .iter()
            .filter(|ticket| ticket.owner == Self::env().caller() && ticket.status == done)
            .cloned()
            .rev()
            .skip(((page - 1) * (100 as u64)).try_into().unwrap())
            .take(100)
            .collect();
        let count = self
            .data::<Data>()
            .tickets
            .iter()
            .filter(|ticket| ticket.owner == Self::env().caller() && ticket.status == done)
            .count() as u64;

        Ok(PageListTicket {
            count: count.clone(),
            page,
            tickets,
        })
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
        let count_tickets = self.data::<Data>().tickets.iter().count() as u64;
        let count_rafflies = self.data::<Data>().rafflies.iter().count() as u64;

        Ok(InfoContract {
            tx_lunes: self.data::<Data>().tx_lunes,
            count_lotto: count_rafflies,
            count_tickets,
        })
    }
}
pub trait Internal: Storage<Data> {
    /// Check amount payable
    fn check_amount(
        &self,
        transferred_value: Balance,
        num_raffle: Vec<ListNumRaffle>,
    ) -> Result<u64, PSP22Error> {
        let raffle_active = self
            .data::<Data>()
            .rafflies
            .iter()
            .find(|raffle| raffle.status);
        if raffle_active.is_none() {
            return Err(PSP22Error::Custom(LunesError::RaffleNotActive.as_str()));
        }
        let price_total = raffle_active.unwrap().price * (num_raffle.len() as u128);
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
            return Ok(raffle_active.unwrap().raffle_id);
        }
        Err(PSP22Error::Custom(LunesError::BadMintValue.as_str()))
    }

    fn create_ticket(
        &mut self,
        raffle_id: u64,
        to: AccountId,
        num_raffle: Vec<ListNumRaffle>,
        date_block: u64,
    ) -> Result<(), PSP22Error> {
        for vet_num in num_raffle {
            let ticket_id = self.data::<Data>().next_ticket_id;
            let ticket = LunesTicket {
                owner: to,
                game_raffle: vec![
                    vet_num.num_1,
                    vet_num.num_2,
                    vet_num.num_3,
                    vet_num.num_4,
                    vet_num.num_5,
                    vet_num.num_6,
                ],
                date_create: date_block,
                raffle_id,
                status: false,
                value_award: 0_u128,
                hits: 0_u64,
                ticket_id,
            };
            self.data::<Data>().next_ticket_id += 1;
            self.data::<Data>().tickets.push(ticket);
        }

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
