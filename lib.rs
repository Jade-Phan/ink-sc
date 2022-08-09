#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ink_sc {
    use ink_env::AccountId;
    use ink::storage::traits::SpreadAllocate;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct InkSc {
        /// Stores a single `bool` value on the storage.
        owner: AccountId,
        id_to_owner:ink_storage::Mapping<u64, AccountId>,
        owner_tokens: ink_storage::Mapping<AccountId, u8>
    }

    impl InkSc {
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(Self::init)
        }

        fn init(&mut self){
            let caller = Self::env().caller();
            self.id_to_owner.insert(0,&caller);
            self.owner_tokens.insert(&caller,0);
            self.caller = caller;
        }

        /// A message that can be called on instantiated contracts.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
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
        fn default_works() {
            let ink_sc = InkSc::default();
            assert_eq!(ink_sc.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut ink_sc = InkSc::new(false);
            assert_eq!(ink_sc.get(), false);
            ink_sc.flip();
            assert_eq!(ink_sc.get(), true);
        }
    }
}
