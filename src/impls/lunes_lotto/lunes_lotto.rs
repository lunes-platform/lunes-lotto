use crate::impls::lunes_lotto::data::{ Data, LunesLotto, LunesTicket, LunesError };
use openbrush::{
    modifiers,
    traits::{ AccountId, Balance, Storage },
    contracts::{ ownable::OwnableError, traits::psp22::PSP22Error },
};
use ink::env::hash::Keccak256;
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
        self.do_raffle(raffle_id, caller, num_rifle.clone(), date_block)?;
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
        Ok(())
    }
    /// Do Raffle in the date
    #[ink(message)]
    fn do_raffle_lotto(&mut self) -> Result<(), LunesError> {
        let index = self
            .data::<Data>()
            .rafflies.iter()
            .position(|raffle| raffle.status == true)
            .unwrap();
        let date_raffle = self.data::<Data>().rafflies[index].date_raffle;
        let date_block = Self::env().block_timestamp();
        if date_block < date_raffle {
            return Err(LunesError::DrawNotStarted);
        }
        //todo call do_raffle
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
pub trait Internal: Storage<Data>{
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
                let mut vec_: Vec<(u32, u32, u32, u32, u32, u32)> = Vec::new();
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

    fn do_raffle(
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
            let seed = 1;
            let muil = 2;
            let increment = 3;
            let modulo =2;

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
                // Renova as roles para garantir exclusividade
                roles = self.roles_linear();
                seed = roles[0];
            }
        }

        result
    }    
    
}
