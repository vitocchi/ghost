#![no_std]
extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate serde;
extern crate hex;
use eng_wasm::*;
use eng_wasm_derive::pub_interface;
mod accounts;
use accounts::{Id, Pass, Accounts};

/*
 Encrypted state keys
*/
static SECRET_ACCOUNTS: &str = "secret_accounts";

// Public struct Contract which will consist of private and public-facing secret contract functions
pub struct Contract;

// Private functions accessible only by the secret contract
impl Contract {
    fn get_accounts() ->  Accounts {
        read_state!(SECRET_ACCOUNTS).unwrap()
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
        let mut accounts = Self::get_accounts();
        let successed = match accounts.registor(id, pass) {
            Ok(_) => {
                write_state!(SECRET_ACCOUNTS => accounts);
                true
            },
            Err(_) => {
                false
            },
        };
       return successed
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
    
    fn authorize(id: Id, pass: Pass) -> bool {
        let accounts = Self::get_accounts();
        accounts.authorize(id, pass) == Ok(())
    }

    fn is_exist(id: Id) -> bool {
        let accounts = Self::get_accounts();
        accounts.is_exist(id)
    }

    fn get_account_ids() -> Vec<Id> {
        let mut accounts = Self::get_accounts();
        accounts.retrieve_all_account_ids()
    }
}