use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Pet::Table)
                    .if_not_exists()
                    .col(pk_auto(Pet::PetId))
                    .col(string(Pet::Name))
                    .col(string_null(Pet::Size))
                    .col(date_null(Pet::Birthday))
                    .col(double_null(Pet::Weight))
                    .col(timestamp(Pet::Timestamp).default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Pet::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Pet {
    Table,
    PetId,
    Name,
    Size,
    Birthday,
    Weight,
    Timestamp
}
