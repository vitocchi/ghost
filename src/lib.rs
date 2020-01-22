#![no_std]
extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate serde;
extern crate hex;
use eng_wasm::*;
use eng_wasm_derive::pub_interface;
pub mod accounts;
use accounts::{Id, Pass, Accounts};

/*
 Encrypted state keys
*/
pub static SECRET_ACCOUNTS: &str = "secret_accounts";

// Public struct Contract which will consist of private and public-facing secret contract functions
pub struct Contract;

impl Contract {
    fn service() -> AccountService<AccountRepository> {
        return AccountService {
            repository: AccountRepository{},
        };
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

trait AccountRepositoryInterface {
    fn get_accounts(&self) -> Accounts;
    fn store_accounts(&self, accounts: Accounts);
}

struct AccountService<T> {
    repository: T,
}

impl <T: AccountRepositoryInterface> AccountService<T> {
    fn registor(self, id: Id, pass: Pass) -> bool{
        let mut accounts = self.repository.get_accounts();
        let successed = match accounts.registor(id, pass) {
            Ok(_) => {
                self.repository.store_accounts(accounts);
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
                [0;, 32]
            }
        }
    }
    */
    
    fn authorize(&self, id: Id, pass: Pass) -> bool {
        let accounts = self.repository.get_accounts();
        accounts.authorize(id, pass) == Ok(())
    }

    fn is_exist(&self, id: Id) -> bool {
        let accounts = self.repository.get_accounts();
        accounts.is_exist(id)
    }

    fn get_account_ids(&self) -> Vec<Id> {
        let mut accounts = self.repository.get_accounts();
        accounts.retrieve_all_account_ids()
    }
}