use crate::impls::lunes_lotto::data::{ Data, LunesLotto, LunesTicket, LunesError };
use ink::env::hash::Keccak256;
use openbrush::{
    modifiers,
    traits::{ AccountId, Balance, Storage },
    contracts::{ ownable::OwnableError, traits::psp22::PSP22Error },
};
use ink_prelude::vec::Vec;
use ink_prelude::vec;
use openbrush::contracts::{
    ownable,
    ownable::only_owner,
    reentrancy_guard,
    reentrancy_guard::non_reentrant,
};
use super::data::ListNumRaffle;
#[openbrush::trait_definition]
pub trait LunesLottoImpl: Storage<Data> +
    Storage<reentrancy_guard::Data> +
    Storage<ownable::Data> +
    Internal
{
    /// Payable in LUNES
    #[ink(message, payable)]
    #[modifiers(non_reentrant)]
    fn play_lunes(&mut self, num_rifle: Vec<ListNumRaffle>) -> Result<(), PSP22Error> {
        let caller = Self::env().caller();
        let date_block = Self::env().block_timestamp();
        let raffle_id = self.check_amount(Self::env().transferred_value(), num_rifle.clone())?;
        let mut value_pay = Self::env().transferred_value();
        let tax_lunes = value_pay * 17 / 100;
        value_pay = value_pay - tax_lunes;
        //update Raffle 
        let index = self.data::<Data>().rafflies.iter().position(|ix| ix.raffle_id == raffle_id).unwrap();
        self.data::<Data>().rafflies[index].total_accumulated += value_pay;
        //Send tax_lunes        
        let owner = self.data::<ownable::Data>().owner.get().unwrap().unwrap();
        Self::env()
        .transfer(owner, value_pay)
        .map_err(|_| PSP22Error::Custom(LunesError::WithdrawalFailed.as_str()))?;
        //create Ticket
        self.create_ticket(raffle_id, caller, num_rifle.clone(), date_block)?;
        Ok(())
    }
    /// Create Raffle with Date, Price and Total Accumulated
    #[ink(message, payable)]
    #[openbrush::modifiers(only_owner)]
    fn create_raffle_lotto(
        &mut self,
        date_raffle: u64,
        price: Balance
    ) -> Result<(), OwnableError> {
        let id = self.data::<Data>().next_id;
        self.data::<Data>().rafflies.push(LunesLotto {
            raffle_id: id,
            num_raffle: Vec::new(),
            date_raffle: date_raffle,
            price: price,
            total_accumulated: Self::env().transferred_value(),
            status: false,
        });
        self.data::<Data>().next_id += 1;
        Ok(())
    }
    /// Do Raffle in the date
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn do_raffle_lotto(&mut self) -> Result<(), PSP22Error> {
        let index = self
            .data::<Data>()
            .rafflies.iter()
            .position(|raffle| raffle.status == true)
            .unwrap();
        let date_raffle = self.data::<Data>().rafflies[index].date_raffle;
        let raffle_id = self.data::<Data>().rafflies[index].raffle_id;
        let total_accumulated = self.data::<Data>().rafflies[index].total_accumulated;
        let price_ticket = self.data::<Data>().rafflies[index].price;
        let value_award_2 = total_accumulated * 2 / 100;
        let value_award_3 = total_accumulated * 5 / 100;
        let value_award_4 = total_accumulated * 10 / 100;
        let value_award_5 = total_accumulated * 25 / 100;
        let value_award_6 = total_accumulated - (value_award_2 + value_award_3 + value_award_4 + value_award_5);

        let date_block = Self::env().block_timestamp();
        if date_block < date_raffle {
            return Err(PSP22Error::Custom(LunesError::DrawNotStarted.as_str()));
        }        
        //todo call do_raffle
        self.data::<Data>().rafflies[index].status = false; 
        let num_raffle = self.linear_random_generator();
        self.data::<Data>().rafflies[index].num_raffle = num_raffle.clone();
              
        //find Winner
        let mut winner:   Vec<LunesTicket> = Vec::new();
        let mut winner_2: Vec<LunesTicket> = Vec::new();
        let mut winner_3: Vec<LunesTicket> = Vec::new();
        let mut winner_4: Vec<LunesTicket> = Vec::new();
        let mut winner_5: Vec<LunesTicket> = Vec::new();
        let mut winner_6: Vec<LunesTicket> = Vec::new();

        let tickets_ = self
            .data::<Data>()
            .tickets.iter()
            .filter(|ticket| ticket.raffle_id == raffle_id)
            .collect::<Vec<&LunesTicket>>();
        for w in tickets_.clone() {
            let game_raffle = w.game_raffle.clone();
            let matching_numbers = game_raffle
                .iter()
                .filter(|&num| num_raffle.clone().contains(num))
                .count();

            match matching_numbers {
                2 => {
                    winner_2.push(w.clone());
                }
                3 => {
                    winner_3.push(w.clone());
                }
                4 => {
                    winner_4.push(w.clone());
                }
                5 => {
                    winner_5.push(w.clone());
                }
                6 => {
                    winner_6.push(w.clone());
                }
                _ => {
                    
                }
            }
        }
        //Distribuition of LUNES
        let mut value_award_next = 0;
        let total_per_pay_2 = value_award_2 / winner_2.len() as u128;
        let total_per_pay_3 = value_award_3 / winner_3.len() as u128;
        let total_per_pay_4 = value_award_4 / winner_4.len() as u128;
        let total_per_pay_5 = value_award_5 / winner_5.len() as u128;
        let total_per_pay_6 = value_award_6 / winner_6.len() as u128;
        for  w in  winner_2.iter_mut() {
            w.value_award = total_per_pay_2;
            w.status = false;
            winner.push(w.clone());
        }
        for  w in  winner_3.iter_mut() {
            w.value_award = total_per_pay_3;
            w.status = false;
            winner.push(w.clone());
        }
        for  w in  winner_4.iter_mut() {
            w.value_award = total_per_pay_4;
            w.status = false;
            winner.push(w.clone());
        }
        for  w in  winner_5.iter_mut() {
            w.value_award = total_per_pay_5;
            w.status = false;
            winner.push(w.clone());
        }
        for  w in  winner_6.iter_mut() {
            w.value_award = total_per_pay_6;
            w.status = false;
            winner.push(w.clone());
        }
        if winner_2.len() == 0 {
            value_award_next += total_per_pay_2;
        } else if winner_3.len() == 0 {
            value_award_next += total_per_pay_3;
        }else if winner_4.len() == 0 {
            value_award_next += total_per_pay_4;
        }else if winner_5.len() == 0 {
            value_award_next += total_per_pay_5;
        }else if winner_6.len() == 0 {
            value_award_next += total_per_pay_6;
        }
        self.data::<Data>().winners.extend(winner);
        //Verify next reffer ou create new
        let next_reffer_index = self
            .data::<Data>().rafflies.iter()
            .position(|raffle| raffle.date_raffle > date_raffle)
            .unwrap();
        if next_reffer_index != 0 {
            self.data::<Data>().rafflies[next_reffer_index].status = true;
        }else{
            let next_id = self.data::<Data>().next_id;
            self.data::<Data>().rafflies.push(LunesLotto {
                raffle_id: next_id,
                num_raffle: Vec::new(),
                date_raffle: date_raffle+ 259343000,
                price: price_ticket,
                total_accumulated: value_award_next,
                status: true,
            });
            self.data::<Data>().next_id += 1;
        }        

        Ok(())
    }
    /// List all Games the user has played
    #[ink(message)]
    fn my_games(&mut self, page: u64) -> Result<Vec<LunesTicket>, PSP22Error> {
        let games = self
            .data::<Data>()
            .tickets.iter()
            .filter(|ticket| ticket.owner == Self::env().caller())
            .cloned()
            .skip(((page - 1) * (100 as u64)).try_into().unwrap())
            .take(100)
            .collect();

        Ok(games)
    }
    /// Payable in LUNES to ticket
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn payment(&mut self) -> Result<(), PSP22Error> {
        if
            let Some(_) = self
                .data::<Data>()
                .winners.iter()
                .find(|winner| winner.owner == Self::env().caller() && winner.status)
        {
            let index = self
                .data::<Data>()
                .winners.iter()
                .position(|winner| winner.owner == Self::env().caller())
                .unwrap();
            self.data::<Data>().winners[index].status = false;
            let caller = Self::env().caller();
            Self::env().transfer(caller, self.data::<Data>().winners[index].value_award).unwrap();
            return Ok(());
        }
        Err(PSP22Error::Custom(LunesError::WithdrawalFailed.as_str()))
    }
    /// List all Raffles
    #[ink(message)]
    fn all_raffle(&mut self, raffle_id: u64, page: u64) -> Result<Vec<LunesLotto>, PSP22Error> {
        let games = self
            .data::<Data>()
            .rafflies.iter()
            .filter(|riff| riff.raffle_id == raffle_id)
            .cloned()
            .skip(((page - 1) * (100 as u64)).try_into().unwrap())
            .take(100)
            .collect();
        Ok(games)
    }
    /// Get Winner Raffle by id
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn winner_raffle(&mut self, raffle_id: u64) -> Result<Vec<LunesTicket>, PSP22Error> {
        let winners = self
            .data::<Data>()
            .winners.iter()
            .filter(|winner| winner.raffle_id == raffle_id)
            .cloned()
            .collect();
        return Ok(winners);
    }
}
pub trait Internal: Storage<Data> {
    /// Check amount payable
    fn check_amount(
        &self,
        transferred_value: Balance,
        num_raffle: Vec<ListNumRaffle>
    ) -> Result<u64, PSP22Error> {
        let raffle_active = self
            .data::<Data>()
            .rafflies.iter()
            .find(|raffle| raffle.status);
        let price_total = raffle_active.unwrap().price * (num_raffle.len() as u128);
        if price_total == transferred_value {
            for vet_num in num_raffle {
                let mut vec_: Vec<(u64, u64, u64, u64, u64, u64)> = Vec::new();
                vec_.push((
                    vet_num.num_1.clone(),
                    vet_num.num_2,
                    vet_num.num_3,
                    vet_num.num_4,
                    vet_num.num_5,
                    vet_num.num_6,
                ));
                for x in vec_.clone() {
                    let u = vec_.iter().any(|y| x == *y);
                    if u {
                        return Err(PSP22Error::Custom(LunesError::BadMintValue.as_str()));
                    }
                }

                let is_exist_zero = vec_
                    .iter()
                    .any(
                        |x| (x.0 == 0 || x.1 == 0 || x.2 == 0 || x.3 == 0 || x.4 == 0 || x.5 == 0)
                    );
                if is_exist_zero {
                    return Err(PSP22Error::Custom(LunesError::BadMintValue.as_str()));
                }

                let is_exist_num_invalid = vec_
                    .iter()
                    .any(|x| (x.0 > 60 || x.1 > 60 || x.2 > 60 || x.3 > 0 || x.4 > 0 || x.5 > 0));
                if is_exist_num_invalid {
                    return Err(PSP22Error::Custom(LunesError::BadMintValue.as_str()));
                }
            }
            return Ok(raffle_active.unwrap().raffle_id);
        }
        return Err(PSP22Error::Custom(LunesError::BadMintValue.as_str()));
    }

    fn create_ticket(
        &mut self,
        raffle_id: u64,
        to: AccountId,
        num_raffle: Vec<ListNumRaffle>,
        date_block: u64
    ) -> Result<(), PSP22Error> {
        for vet_num in num_raffle {
            let ticket = LunesTicket {
                owner: to,
                game_raffle: vec![
                    vet_num.num_1,
                    vet_num.num_2,
                    vet_num.num_3,
                    vet_num.num_4,
                    vet_num.num_5,
                    vet_num.num_6
                ],
                date_create: date_block,
                raffle_id,
                status: false,
                value_award: 0 as u128,
            };
            self.data::<Data>().tickets.push(ticket);
        }

        Ok(())
    }

    fn roles_linear(&mut self) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();

        loop {
            let seed = self.random();
            let muil = self.random();
            let increment = self.random();
            let modulo = self.random();

            match (muil > modulo, increment > modulo, seed > modulo) {
                (true, _, _) => {
                    continue;
                }
                (_, true, _) => {
                    continue;
                }
                (_, _, true) => {
                    continue;
                }
                _ => {
                    result.push(seed);
                    result.push(muil);
                    result.push(increment);
                    result.push(modulo);
                    break;
                }
            }
        }

        result
    }
    fn linear_random_generator(&mut self) -> Vec<u64> {
        const SIZE: usize = 6;
        let mut result: Vec<u64> = Vec::new();
        let mut roles = self.roles_linear();
        let mut seed = roles[0];

        while result.len() < SIZE {
            seed = roles[1].wrapping_mul(seed).wrapping_add(roles[2]) % roles[3];

            if !result.contains(&seed) && seed != 0 {
                result.push(seed);
            } else {
                // remove roles of new duplicate seed
                roles = self.roles_linear();
                seed = roles[0];
            }
        }

        result
    }
    /// Generates a seed based on the list of players and the block number and timestamp
    fn seed(&self) -> u64 {
        let hash = Self::env().hash_encoded::<Keccak256, _>(
            &self.data().tickets.last().unwrap().owner
        );
        let num = u64::from_be_bytes(hash[0..8].try_into().unwrap());
        let timestamp = Self::env().block_timestamp();
        let block_number = Self::env().block_number() as u64;

        num ^ timestamp ^ block_number
    }
    /// Pseudo random number generator between 1 and 60
    fn random(&self) -> u64 {
        let mut x = self.seed();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;

        // Map the random number to the range [1, 60]
        ((x % 60) + 1) as u64
    }
}
