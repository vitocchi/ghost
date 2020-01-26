use eng_wasm::*;
use serde::{Deserialize, Serialize};

pub type Id = String;
pub type Pass = String;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Account {
    pub id: Id,
    pub pass: Pass,
}

impl Account {
    pub fn new(id: Id, pass: Pass) -> Account {
        Account { id: id, pass: pass }
    }
}
