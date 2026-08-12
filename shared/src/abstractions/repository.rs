use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    type Id: Send + Sync;
    type Entity: Send + Sync;
   async  fn find_by_id(&self, id: Self::Id) -> Result<Self::Entity, Box<dyn std::error::Error + Send + Sync>>;
}