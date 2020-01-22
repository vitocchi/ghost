#![no_std]
extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate serde;
extern crate hex;
use eng_wasm::*;
use eng_wasm_derive::pub_interface;
mod account_service;
use account_service::{AccountService, AccountRepositoryInterface};
mod account;
use account::{Id, Pass, Account};

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
    fn get(&self, id: &Id) -> Option<Account> {
        match read_state!(id) {
            Some(pass) => Some(Account::new(id.to_string(), pass)),
            None => None,
        }
    }

    fn save(&self, account: &Account) {
        write_state!(&(account.id) => account.pass.clone());
    }
}

// Public trait defining public-facing secret contract functions
#[pub_interface]
pub trait ContractInterface {
    fn ping() -> String;
    fn registor(id: Id, pass: Pass) -> bool;
    // fn registor_without_pass(id: Id) -> Pass;
    fn authorize(id: Id, pass: Pass) -> bool;
    fn is_exist(id: Id) -> bool;
} 

// Implementation of the public-facing secret contract functions defined in the ContractInterface
// trait implementation for the Contract struct above
impl ContractInterface for Contract {
    #[no_mangle]
    fn ping() -> String {
        "pong".to_string()
    }
    #[no_mangle]
    fn registor(id: Id, pass: Pass) -> bool {
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
}