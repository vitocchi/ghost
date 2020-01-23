#![no_std]
extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate serde;
extern crate hex;
use eng_wasm::*;
mod account_service;
use account_service::{AccountService, AccountRepositoryInterface};
mod account;
use account::{Id, Pass, Account};

// Public struct Contract which will consist of private and public-facing secret contract functions
pub struct SecretAccount{
    service: AccountService<AccountRepository>,
}

impl SecretAccount {
    pub fn new() -> SecretAccount {
        SecretAccount{
            service: AccountService::new(AccountRepository{}),
        }
    }

    pub fn registor(&self, id: Id, pass: Pass) -> bool {
        self.service.registor(id, pass)
    }

    pub fn authorize(&self, id: Id, pass: Pass) -> bool {
        self.service.authorize(id, pass)
    }

    pub fn is_exist(&self, id: Id) -> bool {
        self.service.is_exist(id)
    }
}

static SECRET_ACCOUNT_PREFIX: &str = "secret_account_";

struct AccountRepository;

impl AccountRepositoryInterface for AccountRepository{
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