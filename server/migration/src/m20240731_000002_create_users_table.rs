use sea_orm_migration::prelude::*;

use crate::m20240801_000003_create_role_table::Role;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::Balance).decimal().not_null())
                    .col(ColumnDef::new(User::InviterId).integer().not_null())
                    .col(ColumnDef::new(User::InviteCount).integer().not_null())
                    .col(ColumnDef::new(User::InviteRebateTotal).decimal().not_null())
                    .col(ColumnDef::new(User::RoleId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role_id")
                            .from(User::Table, User::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Username,
    Password,
    Balance,
    InviterId,
    InviteCount,
    InviteRebateTotal,
    CreatedAt,
    RoleId,
}
