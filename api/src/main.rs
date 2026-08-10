
use entity::entities::user;
use sea_orm::{Database, DatabaseConnection, DbErr, EntityTrait};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    println!("Hello, world!");

    // Thay protocol:// bằng postgres://, mysql:// hoặc sqlite:// tương ứng
    let db: DatabaseConnection = Database::connect("postgres://postgres:123456@localhost/postgres").await?;

    let users = user::Entity::find().all(&db).await?;
    
    println!("{:?}",users);

    println!("Kết nối thành công!");

    Ok(())
}