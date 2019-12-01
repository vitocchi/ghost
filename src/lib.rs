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

#[derive(Serialize, Deserialize, std::fmt::Debug)]
struct SecretAccounts (HashMap<Id, Pass>);

impl SecretAccounts {
    fn registor(&mut self, id:Id, pass: Pass) -> Result<(), &'static str> {
        if self.is_exist(id) {
            return Err("id is already used");
        } else {
            self.0.insert(id, pass);
            return Ok(());
        }
    }
    fn authorize(&self, id: Id, pass: Pass) -> bool {
        self.0.get(&id) == Some(&pass)
    }
    fn is_exist(&self, id: Id) -> bool {
        self.0.get(&id) != None
    }
    fn retrieve_all_account_ids(&mut self) -> Vec<Id> {
        let mut vec = Vec::new();
        for (id, _) in self.0.drain() {
            vec.push(id);
        }
        return vec;
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
    fn registor(id: Id, pass: Pass) -> bool;
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
    
    fn authorize(id: Id, pass: Pass) -> bool {
        let accounts = Self::get_accounts();
        accounts.authorize(id, pass)
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

#[cfg(test)]
mod tests {
    use super::*;
    use eng_wasm::Vec;
    #[test]
    fn registor_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let mut sa = SecretAccounts(HashMap::new());
        assert!(sa.registor(id, pass).is_ok());
        assert!(sa.0.contains_key(&id));
        assert!(sa.registor(id, pass).is_err());
    }

    #[test]
    fn is_exist_test() {
        let id = [0; 32];
        let pass = [1; 32];

        let not_exist_id = [2; 32];
        let mut hm = HashMap::new();
        hm.insert(id, pass);
        let sa = SecretAccounts(hm);
        assert!(sa.is_exist(id));
        assert!(!sa.is_exist(not_exist_id));
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
        assert!(!sa.authorize(id, wrong_pass));
    }

    #[test]
    fn registor_and_authorize_success_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let mut sa = SecretAccounts(HashMap::new());
        assert!(sa.registor(id, pass).is_ok());
        assert!(sa.authorize(id, pass));
    }

    #[test]
    fn registor_and_authorize_fail_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let mut sa = SecretAccounts(HashMap::new());
        assert!(sa.registor(id, pass).is_ok());

        let wrong_id = [2; 32];
        let wrong_pass = [3; 32];
        assert!(!sa.authorize(wrong_id, wrong_pass));
    }

    #[test]
    fn retrieve_all_account_ids_test() {
        let id = [0; 32];
        let pass = [1; 32];

        let second_id = [2; 32];
        let second_pass = [3; 32];
        let mut hm = HashMap::new();
        hm.insert(id,pass);
        hm.insert(second_id,second_pass);

        let mut sa = SecretAccounts(hm);
        let ids = sa.retrieve_all_account_ids();
        let mut expected_ids = Vec::new();
        expected_ids.push(second_id);
        expected_ids.push(id);
        assert_eq!(ids, expected_ids);
    }
}
