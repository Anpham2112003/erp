
use crate::shared::abstractions::repository::Repository;
use crate::auth::domain::entities::account::Account;


pub trait AccountRepository :Repository<Id = u32,Entity = Account> {
    fn find_by_email(&self, email: &str) -> Result<Account, Box<dyn std::error::Error>>;
}