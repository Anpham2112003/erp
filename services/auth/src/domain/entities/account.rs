use shared::abstractions::entity::Entity;
pub struct Account {
    
}
impl Entity for Account  {
    fn convert_to_sea_orm<T:sea_orm::prelude::ActiveModelTrait>(&self) ->   T {
        todo!()
    }
}