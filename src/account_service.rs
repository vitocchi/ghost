pub mod account;
use account::*;
use eng_wasm::*;
use std::error::*;
use std::fmt;
use std::result;

pub struct AccountService<T> {
    repository: T,
}

pub trait AccountRepositoryInterface {
    fn get(&self, id: &Id) -> Option<Account>;
    fn get_all(&self) -> Vec<Account>;
    fn save(&self, account: Account);
}

type ServiceResult<T> = result::Result<T, ServiceError>;

#[derive(Debug, PartialEq)]
pub enum ServiceError {
    AccountAlreadyExists,
    AuthorizeFailed,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServiceError::AccountAlreadyExists => f.write_str("AccountAlreadyExists"),
            ServiceError::AuthorizeFailed => f.write_str("AuthorizeFailed"),
        }
    }
}

impl Error for ServiceError {
    fn description(&self) -> &str {
        match *self {
            ServiceError::AccountAlreadyExists => "Account already exists",
            ServiceError::AuthorizeFailed => "Authorize failed",
        }
    }
}

impl<T: AccountRepositoryInterface> AccountService<T> {
    pub fn new(repository: T) -> AccountService<T> {
        AccountService {
            repository: repository,
        }
    }

    pub fn registor(&self, id: Id, pass: Pass) -> ServiceResult<()> {
        match self.repository.get(&id) {
            Some(_) => ServiceResult::Err(ServiceError::AccountAlreadyExists),
            None => {
                let a = Account::new(id, pass);
                self.repository.save(a);
                Ok(())
            }
        }
    }

    pub fn authorize(&self, id: Id, pass: Pass) -> ServiceResult<Account> {
        match self.repository.get(&id) {
            Some(a) => {
                if a.pass == pass {
                    return Ok(a);
                } else {
                    return Err(ServiceError::AuthorizeFailed);
                }
            }
            None => Err(ServiceError::AuthorizeFailed),
        }
    }

    pub fn is_exist(&self, id: Id) -> bool {
        self.repository.get(&id).is_some()
    }

    pub fn get_all_ids(&self) -> Vec<Id> {
        let accounts = self.repository.get_all();
        let mut ids: Vec<Id> = Vec::new();
        for a in accounts {
            ids.push(a.id);
        }
        return ids;
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
        fn get(&self, id: &Id) -> Option<Account> {
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
    }

    #[test]
    fn registor_test() {
        let service = AccountService::new(AccountRepositoryMock {});
        assert_eq!(
            service
                .registor(EMPTY_ID.to_string(), PASS.to_string())
                .unwrap(),
            ()
        );
        assert_eq!(
            service
                .registor(EXIST_ID.to_string(), PASS.to_string())
                .unwrap_err(),
            ServiceError::AccountAlreadyExists
        );
    }

    #[test]
    fn authorize_test() {
        let service = AccountService::new(AccountRepositoryMock {});
        assert_eq!(
            service
                .authorize(EXIST_ID.to_string(), PASS.to_string())
                .unwrap(),
            Account::new(EXIST_ID.to_string(), PASS.to_string())
        );
        assert_eq!(
            service
                .authorize(EXIST_ID.to_string(), WRONG_PASS.to_string())
                .unwrap_err(),
            ServiceError::AuthorizeFailed
        );
        assert_eq!(
            service
                .authorize(EMPTY_ID.to_string(), PASS.to_string())
                .unwrap_err(),
            ServiceError::AuthorizeFailed
        );
    }

    #[test]
    fn is_exist_test() {
        let service = AccountService::new(AccountRepositoryMock {});
        assert_eq!(service.is_exist(EXIST_ID.to_string()), true);
        assert_eq!(service.is_exist(EMPTY_ID.to_string()), false);
    }

    #[test]
    fn get_all_ids_test() {
        let service = AccountService::new(AccountRepositoryMock {});
        let mut vec: Vec<Id> = Vec::with_capacity(1);
        vec.push(EXIST_ID.to_string());
        vec.push(EXIST_ID_2.to_string());
        assert_eq!(service.get_all_ids(), vec);
    }
}
