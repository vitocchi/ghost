#![no_std]
extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate hex;
extern crate serde;
use eng_wasm::*;
mod account_service;
use account_service::{AccountRepositoryInterface, AccountService};
mod account;
use account::{Account, Id};

// Public struct Contract which will consist of private and public-facing secret contract functions
pub struct SecretAccount {
    service: AccountService<AccountRepository>,
}

impl SecretAccount {
    pub fn new() -> SecretAccount {
        SecretAccount {
            service: AccountService::new(AccountRepository {}),
        }
    }

    pub fn registor(&self, id: String, pass: String) -> bool {
        self.service.registor(id, pass).is_ok()
    }

    pub fn authorize(&self, id: String, pass: String) -> bool {
        self.service.authorize(id, pass).is_ok()
    }

    pub fn is_exist(&self, id: String) -> bool {
        self.service.is_exist(id)
    }

    pub fn get_all(&self) -> Vec<String> {
        self.service.get_all_ids()
    }
}

static SECRET_ACCOUNT_PREFIX: &str = "secret_account_";

struct AccountRepository;

impl AccountRepositoryInterface for AccountRepository {
    fn get(&self, id: &Id) -> Option<Account> {
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
        read_state!(SECRET_ACCOUNT_PREFIX)
    }

    fn write(accounts: Vec<Account>) {
        write_state!(SECRET_ACCOUNT_PREFIX => accounts);
    }
}
