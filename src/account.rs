use eng_wasm::*;
use serde::{Serialize, Deserialize};

pub type Id = String;
pub type Pass = String;

#[derive(Serialize, Deserialize)]
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