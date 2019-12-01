#![no_std]
extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate serde;
extern crate hex;
use eng_wasm::*;
use eng_wasm_derive::pub_interface;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/*
 Encrypted state keys
*/
static SECRET_ACCOUNTS: &str = "secret_accounts";

// Public struct Contract which will consist of private and public-facing secret contract functions
pub struct Contract;

type Id = [u8; 32];
type Pass = [u8; 32];

#[derive(Serialize, Deserialize)]
struct SecretAccounts (HashMap<Id, Pass>);

impl SecretAccounts {
    fn registor(&mut self, id:Id, pass: Pass) {
        self.0.insert(id, pass);
    }
    fn authorize(&self, id: Id, pass: Pass) -> bool {
        self.0.get(&id) == Some(&pass)
    }
}

// Private functions accessible only by the secret contract
impl Contract {
    fn get_accounts() ->  SecretAccounts {
        read_state!(SECRET_ACCOUNTS).unwrap()
    }
}

// Public trait defining public-facing secret contract functions
#[pub_interface]
pub trait ContractInterface {
    fn registor(id: Id, pass: Pass);
    fn authorize(id: Id, pass: Pass) -> bool;
}

// Implementation of the public-facing secret contract functions defined in the ContractInterface
// trait implementation for the Contract struct above
impl ContractInterface for Contract {
    #[no_mangle]
    fn registor(id: Id, pass: Pass) {
        let mut accounts = Self::get_accounts();
        accounts.registor(id, pass);
        write_state!(SECRET_ACCOUNTS => accounts);
    }
    
    fn authorize(id: Id, pass: Pass) -> bool {
        let accounts = Self::get_accounts();
        accounts.authorize(id, pass)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn registor_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let mut sa = SecretAccounts(HashMap::new());
        sa.registor(id, pass);
        assert!(sa.0.contains_key(&id));
    }

    #[test]
    fn authorize_success_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let mut hm = HashMap::new();
        hm.insert(id, pass);
        let sa = SecretAccounts(hm);
        assert!(sa.authorize(id, pass));
    }

    #[test]
    fn authorize_fail_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let mut hm = HashMap::new();
        hm.insert(id, pass);
        let sa = SecretAccounts(hm);

        let wrong_id = [2; 32];
        let wrong_pass = [3; 32];
        assert!(!sa.authorize(wrong_id, wrong_pass));
    }

    #[test]
    fn registor_and_authorize_success_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let mut sa = SecretAccounts(HashMap::new());
        sa.registor(id, pass);
        assert!(sa.authorize(id, pass));
    }

    #[test]
    fn registor_and_authorize_fail_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let mut sa = SecretAccounts(HashMap::new());
        sa.registor(id, pass);

        let wrong_id = [2; 32];
        let wrong_pass = [3; 32];
        assert!(!sa.authorize(wrong_id, wrong_pass));
    }
}
