use eng_wasm::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Account {
    pub id: String,
    pub pass: String,
}

impl Account {
    pub fn new(id: String, pass: String) -> Account {
        Account { id: id, pass: pass }
    }
}
