use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{EntityTrait, Set};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the table
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Role::Name).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(Role::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Seed the data
        let db = manager.get_connection();
        let admin = role::ActiveModel {
            name: Set("admin".to_owned()),
            ..Default::default()
        };
        let user = role::ActiveModel {
            name: Set("user".to_owned()),
            ..Default::default()
        };
        role::Entity::insert_many(vec![admin, user])
            .exec(db)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Role {
    #[sea_orm(iden = "roles")]
    Table,
    Id,
    Name,
    CreatedAt,
}

mod role {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "roles")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)]
        pub id: i32,
        pub name: String,
        pub created_at: DateTime,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
