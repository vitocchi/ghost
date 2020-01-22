use eng_wasm::*;
use serde::{Serialize, Deserialize};

pub type Id = String;
pub type Pass = String;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct Account {
    pub id: Id, 
    pub pass: Pass
}

impl Account {
    pub fn new(id: Id, pass: Pass) -> Account {
        Account{
            id: id,
            pass: pass,
        }
    }
}


/*
impl Accounts {
    pub fn registor(&mut self, id:Id, pass: Pass) -> Result<(), &'static str> {
        if self.is_exist(id) {
            return Err("id is already used");
        } else {
            self.0.insert(id, pass);
            return Ok(());
        }
    }
    /*
    fn registor_without_pass(&mut self, id:Id) -> Result<(Pass), &'static str> {
        if self.is_exist(id) {
            return Err("id is already used");
        }
        let mut pass: Pass = [0; 32];
        Rand::gen_slice(&mut pass);
        match self.registor(id, pass) {
            Ok(_) => {
                Ok(pass)
            },
            Err(e) => Err(e),
        }
    }
    */
    pub fn authorize(&self, id: Id, pass: Pass) -> Result<(), &'static str> {
        if self.0.get(&id) == Some(&pass) {
            return Ok(());
        }
        Err("authorization failed")
    }
    pub fn reset_pass(&mut self, id:Id, pass: Pass, new_pass: Pass) -> Result<(), &'static str> {
        match self.authorize(id, pass) {
            Ok(()) => {
                self.0.entry(id).and_modify(|e| {*e = new_pass});
                Ok(())
            },
            Err(err) => {Err(err)}
        }
    }
    pub fn is_exist(&self, id: Id) -> bool {
        self.0.get(&id) != None
    }
    pub fn retrieve_all_account_ids(&mut self) -> Vec<Id> {
        let mut vec = Vec::new();
        for (id, _) in self.0.drain() {
            vec.push(id);
        }
        return vec;
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
        let mut a = Accounts(HashMap::new());
        assert!(a.registor(id, pass).is_ok());
        assert!(a.0.contains_key(&id));
        assert!(a.registor(id, pass).is_err());
    }
    /*
    #[test]
    fn registor_without_pass_test() {
        let id = [0; 32];
        let wrong_pass = [1; 32];
        let mut a = Accounts(HashMap::new());
        match a.registor_without_pass(id) {
            Ok(pass) => { 
                assert!(a.authorize(id, pass).is_ok());
                assert!(a.authorize(id, wrong_pass).is_err());
                assert!(a.registor_without_pass(id).is_err());
            },
            Err(err) => {
                panic!(err);
            }
        };
    }
    */

    #[test]
    fn is_exist_test() {
        let id = [0; 32];
        let pass = [1; 32];

        let not_exist_id = [2; 32];
        let mut hm = HashMap::new();
        hm.insert(id, pass);
        let a = Accounts(hm);
        assert!(a.is_exist(id));
        assert!(!a.is_exist(not_exist_id));
    }

    #[test]
    fn authorize_success_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let mut hm = HashMap::new();
        hm.insert(id, pass);
        let a = Accounts(hm);
        assert!(a.authorize(id, pass).is_ok());
    }

    #[test]
    fn authorize_fail_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let mut hm = HashMap::new();
        hm.insert(id, pass);
        let a = Accounts(hm);

        let wrong_id = [2; 32];
        let wrong_pass = [3; 32];
        assert!(a.authorize(wrong_id, wrong_pass).is_err());
        assert!(a.authorize(id, wrong_pass).is_err());
    }

    #[test]
    fn registor_and_authorize_success_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let mut a = Accounts(HashMap::new());
        assert!(a.registor(id, pass).is_ok());
        assert!(a.authorize(id, pass).is_ok());
    }

    #[test]
    fn registor_and_authorize_fail_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let mut a = Accounts(HashMap::new());
        assert!(a.registor(id, pass).is_ok());

        let wrong_id = [2; 32];
        let wrong_pass = [3; 32];
        assert!(a.authorize(wrong_id, wrong_pass).is_err());
    }

    #[test]
    fn reset_pass_test() {
        let id = [0; 32];
        let pass = [1; 32];
        let new_pass = [3; 32];
        let wrong_pass = [4; 32];
        let mut a = Accounts(HashMap::new());
        assert!(a.registor(id, pass).is_ok());
        assert!(a.authorize(id, pass).is_ok());
        assert!(a.authorize(id, new_pass).is_err());
        assert!(a.reset_pass(id, wrong_pass, new_pass).is_err());
        assert!(a.reset_pass(id, pass, new_pass).is_ok());
        assert!(a.authorize(id, pass).is_err());
        assert!(a.authorize(id, new_pass).is_ok());
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

        let mut a = Accounts(hm);
        let mut ids = a.retrieve_all_account_ids();
        let mut expected_ids = Vec::new();
        expected_ids.push(id);
        expected_ids.push(second_id);
        assert_eq!(ids.sort(), expected_ids.sort());
    }
}
*/