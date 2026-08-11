use sea_orm::ActiveModelTrait;


pub trait Entity {
    fn convert_to_sea_orm<T:ActiveModelTrait>(&self) ->   T;
   
}