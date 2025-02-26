use sea_orm_migration::{prelude::*, schema::*};
use crate::m20220101_000001_create_pet_table::Pet;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .create_table(
                Table::create()
                    .table(PetPhoto::Table)
                    .if_not_exists()
                    .col(pk_auto(PetPhoto::PhotoId))
                    .col(string(PetPhoto::Link))
                    .col(boolean(PetPhoto::IsDefault))
                    .col(integer(PetPhoto::PetId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pet-photo-pet-id")
                            .from(PetPhoto::Table, PetPhoto::PetId)
                            .to(Pet::Table, Pet::PetId)
                    )
                    .col(timestamp(PetPhoto::Timestamp).default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(PetPhoto::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PetPhoto {
    Table,
    PhotoId,
    Link,
    IsDefault,
    PetId,
    Timestamp
}
