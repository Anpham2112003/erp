use entity::entities::role;
use sea_orm_migration::{prelude::*, sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, TransactionTrait}};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260809_193548_seed_auth_default"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
      let db = _manager.get_connection();

      
      let tran = db.begin().await?;
      
     role::ActiveModel
        {
        name: Set("Admin".to_owned()),
        display_name : Set("Quản trị viên".to_owned()),
        ..Default::default()
      }.insert(&tran).await?;

      

      tran.commit().await?;

      Ok(())
      
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        
     let db = _manager.get_connection();

     let admin = role::Entity::find().filter(role::Column::Name.eq("Admin")).one(db).await?;

     admin.unwrap().delete(db).await?;

     Ok(())
    }
}
