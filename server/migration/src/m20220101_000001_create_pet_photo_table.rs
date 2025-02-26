use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .create_table(
                Table::create()
                    .table(MedicationType::Table)
                    .if_not_exists()
                    .col(pk_auto(MedicationType::MedicationTypeId))
                    .col(string(MedicationType::Name))
                    .col(timestamp(MedicationType::Timestamp).default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(MedicationType::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MedicationType {
    Table,
    MedicationTypeId,
    Name,
    Timestamp,
}
