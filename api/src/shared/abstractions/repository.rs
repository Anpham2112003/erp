
pub trait Repository {
    type Id;
    type Entity;
    fn find_by_id(&self, id: Self::Id) -> Result<Self::Entity, Box<dyn std::error::Error>>;
}