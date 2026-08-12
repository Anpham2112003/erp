use crate::abstractions::entity::Entity;




pub trait Dto {
    fn convert_to_entity<T: Entity>(&self) -> T;
}