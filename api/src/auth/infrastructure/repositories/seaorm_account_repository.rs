use crate::{auth::domain::{entities::account::Account, repositories::account_repository::AccountRepository}, shared::abstractions::repository::Repository, };

pub struct SeaormAccountRepository {
    
}

impl AccountRepository for SeaormAccountRepository {
    fn find_by_email(&self, email: &str) -> Result<Account, Box<dyn std::error::Error>> {
        todo!()
    }

}

impl Repository for SeaormAccountRepository {
    type Id=u32;
    type Entity = Account;
    fn find_by_id(&self, id: Self::Id) -> Result<Account, Box<dyn std::error::Error>> {
        todo!()
    }
}

