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
        self.service.registor(id, pass)
    }

    pub fn authorize(&self, id: String, pass: String) -> bool {
        self.service.authorize(id, pass)
    }

    pub fn is_exist(&self, id: String) -> bool {
        self.service.is_exist(id)
    }
}

static SECRET_ACCOUNT_PREFIX: &str = "secret_account_";

struct AccountRepository;

impl AccountRepositoryInterface for AccountRepository {
    fn get(&self, id: &Id) -> Option<Account> {
        match read_state!(&(eformat!("{}{}", SECRET_ACCOUNT_PREFIX, id))) {
            Some(pass) => Some(Account::new(id.to_string(), pass)),
            None => None,
        }
    }

    fn save(&self, account: &Account) {
        write_state!(&(eformat!("{}{}", SECRET_ACCOUNT_PREFIX, account.id)) => account.pass.clone());
    }
}
