
use async_trait::async_trait;
use shared::abstractions::repository::Repository;


use crate::domain::entities::account::Account;

#[async_trait]
pub trait AccountRepository: Repository<Id = u32, Entity = Account> {
    async fn find_by_email(&self, email: &str) -> Result<Account, Box<dyn std::error::Error + Send + Sync>>;
}