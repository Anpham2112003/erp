use async_trait::async_trait;
use shared::abstractions::repository::Repository;

use crate::{domain::{entities::account::Account, repositories::account_repository::AccountRepository},  };

pub struct SeaormAccountRepository {
    
}
#[async_trait]
impl AccountRepository for SeaormAccountRepository {
  async fn find_by_email(&self, email: &str) -> Result<Account, Box<dyn std::error::Error+Send+Sync>> {
        todo!()
    }

}

#[async_trait]
impl Repository for SeaormAccountRepository {
    type Id=u32;
    type Entity = Account;
   async fn find_by_id(&self, id: Self::Id) -> Result<Account, Box<dyn std::error::Error+Send+Sync>> {
        todo!()
    }
}

