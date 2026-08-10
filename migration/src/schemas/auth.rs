use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum User{
    Table,
    Id,
    PublicId,
    UserName,
    Email,
    PhoneNumber,
    Gender,
    DateOfBirth,
    Avatar,
    RoleId
}

#[derive(DeriveIden)]
pub  enum Account {
    Table,
    Id,
    UserId,
    Email,
    Phone,
    HashPassword,
    AccountType 
}


#[derive(DeriveIden)]
pub enum EnumAccountType {
    Type,
    None =1,
    Google =2,
    Tiktok =3,
}





#[derive(DeriveIden)]
pub enum Role{
    Table,
    Id,
    Name,
    DisplayName
}


#[derive(DeriveIden)]
pub enum Permission{
    Table,
    Id,
    Name,
    Path
}

#[derive(DeriveIden)]
pub enum RolePermission{
    Table,
    Id,
    RoleId,
    PermissionId
}