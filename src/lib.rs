#![no_std]
extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate hex;
extern crate serde;
mod account;
mod error;
mod ghost_core;
pub use account::Account;
use eng_wasm::*;
pub use error::Error;
use ghost_core::{AccountRepositoryInterface, GhostCore};

// Public struct Contract which will consist of private and public-facing secret contract functions
pub struct Ghost {
    core: GhostCore<AccountRepository>,
}

impl Ghost {
    pub fn new() -> Ghost {
        Ghost {
            core: GhostCore::new(AccountRepository {}),
        }
    }

    pub fn registor(&self, id: String, pass: String) -> Result<(), Error> {
        self.core.registor(id, pass)
    }

    pub fn authorize(&self, id: String, pass: String) -> Result<Account, Error> {
        self.core.authorize(id, pass)
    }

    pub fn is_exist(&self, id: String) -> bool {
        self.core.is_exist(id)
    }

    pub fn get_all_ids(&self) -> Vec<String> {
        self.core.get_all_ids()
    }

    pub fn delete(&self, id: String, pass: String) -> Result<(), Error> {
        self.core.delete(id, pass)
    }
}

static GHOST_ACCOUNT: &str = "GHOST_ACCOUNT";

struct AccountRepository;

impl AccountRepositoryInterface for AccountRepository {
    fn get(&self, id: &str) -> Option<Account> {
        let accounts: Vec<Account> = Self::read();
        return accounts.into_iter().find(|a| &a.id == id);
    }

    fn get_all(&self) -> Vec<Account> {
        Self::read()
    }

    fn save(&self, account: Account) {
        let mut accounts: Vec<Account> = Self::read();
        accounts.push(account);
        Self::write(accounts);
    }

    fn delete_by_id(&self, id: &str) -> Result<(), Error> {
        let mut accounts: Vec<Account> = Self::read();
        let index = accounts.iter().position(|a| &a.id == id);
        match index {
            Some(index) => {
                accounts.remove(index);
                Self::write(accounts);
                return Ok(());
            }
            None => return Err(Error::AccountNotFound),
        }
    }
}

impl AccountRepository {
    fn read() -> Vec<Account> {
        match read_state!(GHOST_ACCOUNT) {
            Some(a) => a,
            None => Vec::new(),
        }
    }

    fn write(accounts: Vec<Account>) {
        write_state!(GHOST_ACCOUNT => accounts);
    }
}
