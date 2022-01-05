#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;

#[ink::contract]
mod staking {
    use ink_storage::collections::HashMap;
    use ink_storage::traits::{PackedLayout, SpreadLayout, StorageLayout};
    use erc20::erc20::{Erc20, Erc20Ref};
    use ink_env::call::FromAccountId;
    use ink_lang::codegen::EmitEvent;

    #[derive(Default, Debug, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Copy, Clone,
        PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    struct Record {
        staked_amount: Balance,
        staked_at: u64,
        unstaked_amount: Balance,
        unstaked_at: u64,
        reward_amount: Balance,
    }

    // #[cfg(not(feature = "ink-as-dependency"))]
    #[ink(storage)]
    pub struct Staking {
        owner: AccountId,
        reward_interval: Balance,
        reward_token_address: AccountId,
        // (AccountId, AccountId) -> (token_addr, user_addr)
        records: HashMap<(AccountId, AccountId), Record>,
        // token_addr -> reward_rates
        reward_rates: HashMap<AccountId, Balance>,
    }

    const DAY_SECONDS: u128 = 60 * 60 * 24;

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Stake {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        amount: Balance,
        #[ink(topic)]
        staked_at: u64,
    }

    #[ink(event)]
    pub struct Unstake {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        amount: Balance,
        #[ink(topic)]
        token_addr: AccountId,
        reward: Balance,
        #[ink(topic)]
        unstaked_at: u64,
    }

    #[ink(event)]
    pub struct WithdrawUnstaked {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        amount: Balance,
        #[ink(topic)]
        withdraw_at: u64,
    }

    #[ink(event)]
    pub struct WithdrawRewards {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        amount: Balance,
        #[ink(topic)]
        withdraw_at: u64,
    }

    #[ink(event)]
    pub struct WithdrawByOwner {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        token_addr: AccountId,
        #[ink(topic)]
        amount: Balance,
        #[ink(topic)]
        new_balance: Balance,
    }

    #[ink(event)]
    pub struct SetRewardRate {
        #[ink(topic)]
        token_addr: AccountId,
        #[ink(topic)]
        new_reward_rate: Balance,
    }

    impl Staking {
        #[ink(constructor)]
        pub fn new(reward_token_address: AccountId) -> Self {
            Self {
                owner: Self::env().caller(),
                reward_interval: 365 * 1 * DAY_SECONDS,
                reward_token_address,
                records: HashMap::new(),
                reward_rates: HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn stake(&mut self, token_addr: AccountId, _amount: Balance) -> bool {
            if self.reward_rate(token_addr) <= 0 {
                return false;
            }
            // need to get access to token here
            let mut token: Erc20Ref = FromAccountId::from_account_id(token_addr);
            let mut amount = _amount;
            // if _amount is 0 stake all tokens
            if _amount == 0 {
                amount = token.balance_of(self.env().caller());
            }

            if token.balance_of(self.env().caller()) < amount {
                return false;
            }
            if token.allowance(self.env().caller(), self.token_addr()) < amount {
                return false;
            }

            let block_stamp = self.env().block_timestamp();
            let mut record = self.record_of_or_zero(token_addr, self.env().caller());
            if record.staked_amount > 0 {
                // let reward = calculate_reward(token_addr, self.env().caller(), record.staked_amount);
                let reward = 50;
                record.reward_amount += reward;
            }
            record.staked_amount += amount;
            record.staked_at = block_stamp;

            EmitEvent::<Staking>::emit_event(self.env(), Stake {
                user: self.env().caller(),
                amount,
                staked_at: self.env().block_timestamp(),
            });
            token.transfer_from(self.env().caller(), self.token_addr(), amount);
            true
        }

        #[ink(message)]
        pub fn unstake(&mut self, token_addr: AccountId, amount: Balance) {
            todo!("For users to unstake their staked tokens");
            // Emit Unstake event
        }

        #[ink(message)]
        pub fn withdraw_unstaked(&mut self, token_addr: AccountId, amount: Balance) {
            todo!(
                "For users to withdraw their staked tokens from this contract to the caller's
                address"
            );
            // Emit WithdrawUnstaked Event
        }

        #[ink(message)]
        pub fn withdraw_reward(&mut self, token_addr: AccountId, amount: Balance) {
            todo!("for users to withdraw reward tokens from this contract to the caller's address");
            // Emit WithdrawReward Event
        }

        #[ink(message)]
        pub fn set_reward_rate(&mut self, token_addr: AccountId, reward_rate: Balance) -> bool {
            if self.env().caller() != self.owner {
                return false;
            }

            self.reward_rates.insert(token_addr, reward_rate);

            EmitEvent::<Staking>::emit_event(self.env(), SetRewardRate {
                    token_addr,
                    new_reward_rate: reward_rate,
                });
            true
        }

        #[ink(message)]
        pub fn reward_rate(&self, token_addr: AccountId) -> Balance {
            self.reward_of_or_zero(&token_addr)
        }

        #[ink(message)]
		pub fn token_addr(&self) -> AccountId {
			self.owner
		}

        #[ink(message)]
		pub fn reward_token_addr(&self) -> AccountId {
			self.reward_token_address
		}

        fn reward_of_or_zero(&self, token_addr: &AccountId) -> Balance {
            *self.reward_rates.get(token_addr).unwrap_or(&0)
        }

        fn record_of_or_zero(&mut self, token_addr: AccountId, owner: AccountId) -> &mut Record {
            self.records.entry((token_addr, owner)).or_insert_with(|| Record {
                    staked_amount: 0,
                    staked_at: 0,
                    unstaked_amount: 0,
                    unstaked_at: 0,
                    reward_amount: 0,
            })
        }

        fn calculate_reward(token_addr: AccountId, user: AccountId, amount: Balance) -> Balance {
            todo!(
                "calculate rewards based on the duration of staked tokens,
                staked token amount, reward rate of the staked token, reward interval"
            );
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let contract = Staking::new(AccountId::from([0x2; 32]));
            assert_eq!(contract.reward_token_addr(), AccountId::from([0x2; 32]));
        }

        #[ink::test]
        fn stake_works() {
            let mut staker = Staking::new(AccountId::from([0x2; 32]));
            let mut token = Erc20::new(100);
            let token_address = token.token_addr();
            // println!("{:?}", token_address);
            assert_eq!(token.balance_of(token_address), 100);
            assert!(token.transfer(AccountId::from([0x4; 32]), 10));
            assert_eq!(token.balance_of(AccountId::from([0x4; 32])), 10);
            // println!("staker address {:?}", staker.my_addr());
            // assert!(staker.stake(token_address, 4));
        }

        #[ink::test]
        fn set_reward_rate() {
            let mut contract = Staking::new(AccountId::from([0x2; 32]));
            let reward_rate = 100;
            contract.set_reward_rate(AccountId::from([0x5; 32]), reward_rate);
            assert_eq!(
                reward_rate,
                contract.reward_rate(AccountId::from([0x5; 32]))
            );
        }
    }
}
