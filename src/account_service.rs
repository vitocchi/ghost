use super::account::{Id, Pass, Account};

pub struct AccountService<T> {
    repository: T,
}

pub trait AccountRepositoryInterface {
    fn get(&self, id: &Id) -> Option<Account>;
    fn save(&self, account: &Account);
}

impl <T: AccountRepositoryInterface> AccountService<T> {
    pub fn new(repository: T) -> AccountService<T> {
        AccountService {
            repository: repository,
        }
    }

    pub fn registor(&self, id: Id, pass: Pass) -> bool{
        match self.repository.get(&id) {
            Some(_) => {
                false
            },
            None => {
                let a = Account::new(id, pass);
                self.repository.save(&a);
                true
            }
        }
    }
    
    pub fn authorize(&self, id: Id, pass: Pass) -> bool {
        match self.repository.get(&id) {
            Some(a) => {
                a.pass == pass
            },
            None => {
                false
            }
        }
    }

    pub fn is_exist(&self, id: Id) -> bool {
        self.repository.get(&id).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eng_wasm::*;

    const EXIST_ID: &'static str = "existID";
    const EMPTY_ID: &'static str = "emptyID";
    const PASS: &'static str = "PASS";
    const WRONG_PASS: &'static str = "wrongPASS";

    struct AccountRepositoryMock {}
    impl AccountRepositoryInterface for AccountRepositoryMock {
        fn get(&self, id: &Id) -> Option<Account> {
            match id as &str {
                EXIST_ID => Some(Account::new(EXIST_ID.to_string(), PASS.to_string())),
                _ => None
            }
        }
        fn save(&self, _account: &Account) {}
    }

    #[test]
    fn registor_test() {
        let service = AccountService::new(AccountRepositoryMock{});
        assert_eq!(service.registor(EMPTY_ID.to_string(), PASS.to_string()), true);
        assert_eq!(service.registor(EXIST_ID.to_string(), PASS.to_string()), false);
    }

    #[test]
    fn authorize_test() {
        let service = AccountService::new(AccountRepositoryMock{});
        assert_eq!(service.authorize(EXIST_ID.to_string(), PASS.to_string()), true);
        assert_eq!(service.authorize(EXIST_ID.to_string(), WRONG_PASS.to_string()), false);
        assert_eq!(service.authorize(EMPTY_ID.to_string(), PASS.to_string()), false);
    }

    #[test]
    fn is_exist_test() {
        let service = AccountService::new(AccountRepositoryMock{});
        assert_eq!(service.is_exist(EXIST_ID.to_string()), true);
        assert_eq!(service.is_exist(EMPTY_ID.to_string()), false);
    }
}
    