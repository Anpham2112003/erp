pub trait UnitOfWork {
    fn begin_transaction(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn commit(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn rollback(&self) -> Result<(), Box<dyn std::error::Error>>;
}