use crate::shared::abstractions::entities::entity::Entity;

pub trait Dto {
    fn convert_to_entity<T: Entity>(&self) -> T;
}