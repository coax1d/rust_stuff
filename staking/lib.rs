#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;

#[ink::contract]
mod staking {
    use ink_storage::collections::HashMap;
    use ink_storage::traits::{PackedLayout, SpreadLayout, StorageLayout};

    #[derive(Default, Debug, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    struct Record {
        staked_amount: Balance,
        staked_at: u64,
        unstaked_amount: Balance,
        unstaked_at: u64,
        reward_amount: Balance,
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    #[ink(storage)]
    pub struct Staking {
        owner: AccountId,
        /// The total supply.
        total_supply: Balance,
        /// The balance of each user.
        balances: HashMap<AccountId, Balance>,
        /// Approval spender on behalf of the message's sender.
        allowances: HashMap<(AccountId, AccountId), Balance>,

        reward_interval: Balance,

        reward_address: AccountId,

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
        pub fn new(initial_supply: Balance, reward_address: AccountId) -> Self {
            let caller = Self::env().caller();
            let mut balances = ink_storage::collections::HashMap::new();
            balances.insert(caller, initial_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });

            Self {
                owner: caller,
                total_supply: initial_supply,
                balances,
                allowances: HashMap::new(),
                reward_interval: 365 * 1 * DAY_SECONDS,
                reward_address,
                records: HashMap::new(),
                reward_rates: HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn stake(&mut self, token_addr: AccountId, _amount: Balance) -> bool {
            if self.reward_rate(token_addr) <= 0 {
                return false;
            }
            // let mut token = Erc20(token_addr) // need to get access to token here
            let mut amount = _amount;
            // if _amount is 0 stake all tokens
            if _amount == 0 {
                amount = token.balance_of(self.env().caller());
            }

            if token.balance_of(self.env().caller()) < amount {
                return false;
            }
            if token.allowance(self.env().caller(), self.owner) < amount {
                return false;
            }

            let record = self.record_of_or_zero(&token_addr, &self.owner);
            if record.staked_amount > 0 {
                let reward = calculate_reward(token_addr, self.env().caller(), record.staked_amount);
                record.reward_amount += reward;
            }
            record.staked_amount += amount;
            record.staked_at = self.env().block_timestamp();

            self.env().emit_event(Stake {
                user: self.env().caller(),
                amount,
                staked_at: self.env().block_timestamp(),
            });
            token.transfer_from(self.env().caller(), self.owner, amount);
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

            self.env().emit_event(SetRewardRate {
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
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), value);

            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            true
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_of_or_zero(&owner, &spender)
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let allowance = self.allowance(from, self.env().caller());

            if allowance < value {
                return false;
            }

            self.allowances
                .insert((from, self.env().caller()), allowance - value);
            self.transfer_from_to(from, to, value)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            self.transfer_from_to(self.env().caller(), to, value)
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false;
            }

            // Update the sender's balance.
            self.balances.insert(from, from_balance - value);

            // Update the receiver's balance.
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(to, to_balance + value);

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });

            true
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }

        fn allowance_of_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            *self.allowances.get(&(*owner, *spender)).unwrap_or(&0)
        }

        fn reward_of_or_zero(&self, token_addr: &AccountId) -> Balance {
            *self.reward_rates.get(token_addr).unwrap_or(&0)
        }

        /*struct Record {
            staked_amount: Balance,
            staked_at: Balance,
            unstaked_amount: Balance,
            unstaked_at: Balance,
            reward_amount: Balance,
        }*/
        fn record_of_or_zero(&self, token_addr: &AccountId, owner: &AccountId) -> &Record {
            self.records.get(&(*token_addr, *owner)).unwrap_or(&
                Record {
                    staked_amount: 0,
                    staked_at: 0,
                    unstaked_amount: 0,
                    unstaked_at: 0,
                    reward_amount: 0,
                }
            )
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
            let contract = Staking::new(777, AccountId::from([0x2; 32]));
            assert_eq!(contract.total_supply(), 777);
            assert_eq!(contract.reward_address, AccountId::from([0x2; 32]));
        }

        #[ink::test]
        fn balance_works() {
            let contract = Staking::new(100, AccountId::from([0x2; 32]));
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Staking::new(100, AccountId::from([0x2; 32]));
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert!(contract.transfer(AccountId::from([0x0; 32]), 10));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
            assert!(!contract.transfer(AccountId::from([0x0; 32]), 100));
        }

        #[ink::test]
        fn transfer_from_works() {
            let mut contract = Staking::new(100, AccountId::from([0x2; 32]));
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            contract.approve(AccountId::from([0x1; 32]), 20);
            contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 10);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
        }

        #[ink::test]
        fn set_reward_rate() {
            let mut contract = Staking::new(1000, AccountId::from([0x2; 32]));
            let reward_rate = 100;
            contract.set_reward_rate(AccountId::from([0x5; 32]), reward_rate);
            assert_eq!(
                reward_rate,
                contract.reward_rate(AccountId::from([0x5; 32]))
            );
        }
    }
}
