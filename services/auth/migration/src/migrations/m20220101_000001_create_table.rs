
use sea_orm_migration::{
    prelude::*,
    schema::*,
    sea_query::extension::postgres::Type,
};

use crate::schemas::{auth::*, share::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ============================================================
        // ENUM
        // ============================================================

        manager
            .create_type(
                Type::create()
                    .as_enum(EnumAccountType::Type)
                    .values([
                        EnumAccountType::None,
                        EnumAccountType::Google,
                        EnumAccountType::Tiktok,
                    ])
                    .to_owned(),
            )
            .await?;

        // ============================================================
        // USER
        // ============================================================

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(uuid_uniq(User::PublicId))
                    .col(string(User::UserName))
                    .col(string(User::Email))
                    .col(integer(User::RoleId))
                    .col(date_time(User::DateOfBirth))
                    .col(boolean(User::Gender))
                    .col(string(User::Avatar))
                    .col(string(User::PhoneNumber))
                    .add_text_search_colum()
                    .audit_full()
                    .to_owned(),
            )
            .await?;

        // ============================================================
        // ROLE
        // ============================================================

        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(pk_auto(Role::Id))
                    .col(string(Role::Name))
                    .col(string(Role::DisplayName))
                    .to_owned(),
            )
            .await?;

        // ============================================================
        // PERMISSION
        // ============================================================

        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(pk_auto(Permission::Id))
                    .col(string(Permission::Name))
                    .col(string(Permission::Path))
                    .to_owned(),
            )
            .await?;

        // ============================================================
        // ROLE PERMISSION
        // ============================================================

        manager
            .create_table(
                Table::create()
                    .table(RolePermission::Table)
                    .if_not_exists()
                    .col(pk_auto(RolePermission::Id))
                    .col(integer(RolePermission::RoleId))
                    .col(integer(RolePermission::PermissionId))
                    .to_owned(),
            )
            .await?;

        // ============================================================
        // ACCOUNT
        // ============================================================

        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(pk_auto(Account::Id))
                    .col(integer(Account::UserId))
                    .col(string(Account::Email))
                    .col(string(Account::Phone))
                    .col(string(Account::HashPassword))
                    .col(
                        ColumnDef::new(Account::AccountType)
                            .custom(EnumAccountType::Type)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // ============================================================
        // FOREIGN KEYS
        // ============================================================

        // User.RoleId -> Role.Id
        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .from_tbl(User::Table)
                    .from_col(User::RoleId)
                    .to_tbl(Role::Table)
                    .to_col(Role::Id)
                    .to_owned(),
            )
            .await?;

        // RolePermission.RoleId -> Role.Id
        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .from_tbl(RolePermission::Table)
                    .from_col(RolePermission::RoleId)
                    .to_tbl(Role::Table)
                    .to_col(Role::Id)
                    .to_owned(),
            )
            .await?;

        // RolePermission.PermissionId -> Permission.Id
        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .from_tbl(RolePermission::Table)
                    .from_col(RolePermission::PermissionId)
                    .to_tbl(Permission::Table)
                    .to_col(Permission::Id)
                    .to_owned(),
            )
            .await?;

        // Account.UserId -> User.Id
        manager
            .create_foreign_key(
                ForeignKeyCreateStatement::new()
                    .from_tbl(Account::Table)
                    .from_col(Account::UserId)
                    .to_tbl(User::Table)
                    .to_col(User::Id)
                    .to_owned(),
            )
            .await?;

        // ============================================================
        // INDEX
        // ============================================================

        manager
            .create_index(
                Index::create()
                    .table(User::Table)
                    .col(User::Email)
                    .col(User::UserName)
                    .col(User::PhoneNumber)
                    .add_text_search_colum()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Account::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(RolePermission::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Permission::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(User::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Role::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(EnumAccountType::Type)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

