#![cfg_attr(not(feature = "std"), no_std)]

pub use self::fqy::{
    Fqy,
    FqyRef
};

#[ink::contract]
mod fqy {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Fqy {
        uuid:u32
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error{
        NotGood
    }

    impl Fqy {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self { uuid: init_value }
        }
       

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.uuid
        }

        #[ink(message)]
        pub fn set(&mut self) -> Result<()>{
            self.uuid +=1;
            Ok(())
        }
    }

    
}
