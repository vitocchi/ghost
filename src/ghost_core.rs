use super::account::Account;
use super::error::Error;
use eng_wasm::*;

pub struct GhostCore<T> {
    repository: T,
}

pub trait AccountRepositoryInterface {
    fn get(&self, id: &str) -> Option<Account>;
    fn get_all(&self) -> Vec<Account>;
    fn save(&self, account: Account);
    fn delete_by_id(&self, id: &str) -> Result<(), Error>;
}

impl<T: AccountRepositoryInterface> GhostCore<T> {
    pub fn new(repository: T) -> GhostCore<T> {
        GhostCore {
            repository: repository,
        }
    }

    pub fn registor(&self, id: String, pass: String) -> Result<(), Error> {
        match self.repository.get(&id) {
            Some(_) => Result::Err(Error::AccountAlreadyExists),
            None => {
                let a = Account::new(id, pass);
                self.repository.save(a);
                Ok(())
            }
        }
    }

    pub fn authorize(&self, id: String, pass: String) -> Result<Account, Error> {
        match self.repository.get(&id) {
            Some(a) => {
                if a.pass == pass {
                    return Ok(a);
                } else {
                    return Err(Error::AuthorizeFailed);
                }
            }
            None => Err(Error::AuthorizeFailed),
        }
    }

    pub fn is_exist(&self, id: String) -> bool {
        self.repository.get(&id).is_some()
    }

    pub fn get_all_ids(&self) -> Vec<String> {
        let ghosts = self.repository.get_all();
        let mut ids: Vec<String> = Vec::new();
        for a in ghosts {
            ids.push(a.id);
        }
        return ids;
    }

    pub fn delete(&self, id: String, pass: String) -> Result<(), Error> {
        let a = self.authorize(id, pass)?;
        self.repository.delete_by_id(&a.id).unwrap();
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eng_wasm::*;

    const EXIST_ID: &'static str = "existID";
    const EXIST_ID_2: &'static str = "existID_2";
    const EMPTY_ID: &'static str = "emptyID";
    const PASS: &'static str = "PASS";
    const WRONG_PASS: &'static str = "wrongPASS";

    struct AccountRepositoryMock {}
    impl AccountRepositoryInterface for AccountRepositoryMock {
        fn get(&self, id: &str) -> Option<Account> {
            match id as &str {
                EXIST_ID => Some(Account::new(EXIST_ID.to_string(), PASS.to_string())),
                _ => None,
            }
        }
        fn get_all(&self) -> Vec<Account> {
            let mut vec: Vec<Account> = Vec::with_capacity(1);
            vec.push(Account::new(EXIST_ID.to_string(), PASS.to_string()));
            vec.push(Account::new(EXIST_ID_2.to_string(), PASS.to_string()));
            return vec;
        }
        fn save(&self, _account: Account) {}
        fn delete_by_id(&self, _id: &str) -> Result<(), Error> {
            Ok(())
        }
    }

    #[test]
    fn registor_test() {
        let core = GhostCore::new(AccountRepositoryMock {});
        assert_eq!(
            core.registor(EMPTY_ID.to_string(), PASS.to_string())
                .unwrap(),
            ()
        );
        assert_eq!(
            core.registor(EXIST_ID.to_string(), PASS.to_string())
                .unwrap_err(),
            Error::AccountAlreadyExists
        );
    }

    #[test]
    fn authorize_test() {
        let core = GhostCore::new(AccountRepositoryMock {});
        assert_eq!(
            core.authorize(EXIST_ID.to_string(), PASS.to_string())
                .unwrap(),
            Account::new(EXIST_ID.to_string(), PASS.to_string())
        );
        assert_eq!(
            core.authorize(EXIST_ID.to_string(), WRONG_PASS.to_string())
                .unwrap_err(),
            Error::AuthorizeFailed
        );
        assert_eq!(
            core.authorize(EMPTY_ID.to_string(), PASS.to_string())
                .unwrap_err(),
            Error::AuthorizeFailed
        );
    }

    #[test]
    fn is_exist_test() {
        let core = GhostCore::new(AccountRepositoryMock {});
        assert_eq!(core.is_exist(EXIST_ID.to_string()), true);
        assert_eq!(core.is_exist(EMPTY_ID.to_string()), false);
    }

    #[test]
    fn get_all_ids_test() {
        let core = GhostCore::new(AccountRepositoryMock {});
        let mut vec: Vec<String> = Vec::with_capacity(1);
        vec.push(EXIST_ID.to_string());
        vec.push(EXIST_ID_2.to_string());
        assert_eq!(core.get_all_ids(), vec);
    }

    #[test]
    fn delete_test() {
        let core = GhostCore::new(AccountRepositoryMock {});
        assert_eq!(
            core.delete(EMPTY_ID.to_string(), PASS.to_string())
                .unwrap_err(),
            Error::AuthorizeFailed
        );
        assert_eq!(
            core.delete(EXIST_ID.to_string(), PASS.to_string()).unwrap(),
            ()
        );
    }
}
