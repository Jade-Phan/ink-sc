#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ink_sc {
    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct InkSc {
        /// Stores a single `bool` value on the storage.
        owner: AccountId,
        id_to_owner: Mapping<u32, AccountId>,
        owner_tokens: Mapping<AccountId, u32>
    }

    #[ink(event)]
    pub struct Mint{
        #[ink(topic)]
        receiver: AccountId,
        #[ink(topic)]
        token_id:u32,
    }

    #[ink(event)]
    pub struct Transfer{
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to:AccountId,
        #[ink(topic)]
        token_id: u32,
    }

    /// Error that occurred
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not the owner
        NotOwner,
        /// Returned if the account doesnt own the nft token id
        NotOwnedToken,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl InkSc {
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|contract|Self::init(contract))
        }

        fn init(&mut self){
            let caller = Self::env().caller();
            self.owner = caller;
        }

         #[ink(message)]
        pub fn mint(&mut self, receiver: AccountId, token_id: u32) -> Result<()> {
             // only owner can mint
            if Self::env().caller() == self.owner {
                self.id_to_owner.insert(token_id, &receiver);
                if let Some(n) = self.owner_tokens.get(&receiver){
                    self.owner_tokens.insert(receiver, &(n+1));
                } else {
                    self.owner_tokens.insert(receiver, &1);
                }
                self.env().emit_event(Mint{
                    receiver: receiver,
                    token_id: token_id,
                });
                Ok(())
            } else {
                Err(Error::NotOwner)
            }
        }

        fn is_owner_of(&self, token_id: u32, account: &AccountId) -> bool {
            let owner = self.id_to_owner.get(&token_id);
            match owner {
                Some(acc) => return if acc != *account {false} else {true},
                None => false,
            }
        }

        #[ink(message)]
        pub fn transfer(&mut self, from: AccountId, to: AccountId, token_id: u32) -> Result<()> {
            if !self.is_owner_of(token_id, &from){
                return Err(Error::NotOwnedToken)
            }

            // check if the caller is the owner of the token
            if self.env().caller() != from {
                return Err(Error::NotOwnedToken)
            }

            self.id_to_owner.insert(token_id, &to);
            let count_of_from = self.owner_tokens.get(&from).unwrap();
            let count_of_to = self.owner_tokens.get(&to).unwrap_or(0);
            self.owner_tokens.insert(from, &(count_of_from-1));
            self.owner_tokens.insert(to, &(count_of_to+1));
            self.env().emit_event(Transfer{
                from: from,
                to:to,
                token_id: token_id,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn get_owner_of_token(&self, token_id: u32) -> AccountId {
            self.id_to_owner.get(token_id).unwrap()
        }

        #[ink(message)]
        pub fn count_of_owner(&self, account: AccountId) -> u32 {
            self.owner_tokens.get(&account).unwrap()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_initializer() {
            let ink_sc = InkSc::default();
            let token_count = ink_sc.owner_tokens.get(AccountId::from([0x1;32]));

            assert_eq!(ink_sc.owner, AccountId::from([0x1;32]));
            assert_eq!(token_count, None);
        }

        #[ink::test]
        fn mint() {
            //Given
            let mut ink_sc = InkSc::default();
            let account_one = AccountId::from([0x1; 32]);
            let token_id = 95;

            //When
            ink_sc.mint(account_one, token_id).expect("Expected result");

            //Then
            assert_eq!(ink_sc.get_owner_of_token(95),account_one);
        }

        #[ink::test]
        fn transfer() {
            //Given
            let mut ink_sc = InkSc::default();
            ink_env::debug_println!("created new instance at {:?}", ink_sc.owner);
            let account_one = AccountId::from([0x1; 32]);
            let account_two = AccountId::from([0x2; 32]);
            let token_id = 95;

            //When
            ink_sc.mint(account_one, token_id);
            ink_sc.transfer(account_one, account_two, token_id);

            //Then
            assert_eq!(ink_sc.owner_tokens.get(account_one),Some(0));
            assert_eq!(ink_sc.owner_tokens.get(account_two),Some(1));
        }
    }
}
