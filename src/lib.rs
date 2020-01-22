#![no_std]
extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate serde;
extern crate hex;
use eng_wasm::*;
use eng_wasm_derive::pub_interface;
mod account_service;
use account_service::{AccountService, AccountRepositoryInterface};
mod accounts;
use accounts::{Id, Pass, Accounts};

/*
 Encrypted state keys
*/
pub static SECRET_ACCOUNTS: &str = "secret_accounts";

// Public struct Contract which will consist of private and public-facing secret contract functions
pub struct Contract;

// Private functions accessible only by the secret contract
impl Contract {
    fn service() -> AccountService<AccountRepository> {
        AccountService::new(AccountRepository{})
    }
}

struct AccountRepository;

impl AccountRepositoryInterface for AccountRepository{
    fn get_accounts(&self) -> Accounts {
        read_state!(SECRET_ACCOUNTS).unwrap()
    }
    fn store_accounts(&self, accounts: Accounts) {
        write_state!(SECRET_ACCOUNTS => accounts);
    }
}

// Public trait defining public-facing secret contract functions
#[pub_interface]
pub trait ContractInterface {
    fn registor(id: Id, pass: Pass) -> bool;
    // fn registor_without_pass(id: Id) -> Pass;
    fn authorize(id: Id, pass: Pass) -> bool;
    fn is_exist(id: Id) -> bool;
    fn get_account_ids() -> Vec<Id>;
} 

// Implementation of the public-facing secret contract functions defined in the ContractInterface
// trait implementation for the Contract struct above
impl ContractInterface for Contract {
    #[no_mangle]
    fn registor(id: Id, pass: Pass) -> bool{
        Self::service().registor(id, pass)
    }

    /*
    fn registor_without_pass(id: Id) -> Pass {
        let mut accounts = Self::get_accounts();
        match accounts.registor_without_pass(id) {
            Ok(pass) => {
                write_state!(SECRET_ACCOUNTS => accounts);
                pass
            },
            Err(_) => {
                [0; 32]
            }
        }
    }
    */
    
    #[no_mangle]
    fn authorize(id: Id, pass: Pass) -> bool {
        Self::service().authorize(id, pass)
    }


    #[no_mangle]
    fn is_exist(id: Id) -> bool {
        Self::service().is_exist(id)
    }

    #[no_mangle]
    fn get_account_ids() -> Vec<Id> {
        Self::service().get_account_ids()
    }
}