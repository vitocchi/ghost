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
}

static GHOST_ACCOUNT: &str = "GHOST_ACCOUNT";

struct AccountRepository;

impl AccountRepositoryInterface for AccountRepository {
    fn get(&self, id: &str) -> Option<Account> {
        match Self::read() {
            Some(accounts) => accounts.into_iter().find(|a| &a.id == id),
            None => None,
        }
    }

    fn get_all(&self) -> Vec<Account> {
        match Self::read() {
            Some(a) => a,
            None => Vec::new(),
        }
    }

    fn save(&self, account: Account) {
        let mut accounts: Vec<Account> = self.get_all();
        accounts.push(account);
        Self::write(accounts);
    }
}

impl AccountRepository {
    fn read() -> Option<Vec<Account>> {
        read_state!(GHOST_ACCOUNT)
    }

    fn write(accounts: Vec<Account>) {
        write_state!(GHOST_ACCOUNT => accounts);
    }
}
