use sea_orm_migration::{prelude::*, schema::*};
use super::m20220101_000001_create_medication_type_table::MedicationType;
use super::m20220101_000001_create_pet_table::Pet;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(PetMedication::Table)
                    .if_not_exists()
                    .col(pk_auto(PetMedication::MedicationId))
                    .col(integer(PetMedication::MedicationTypeId))
                    .col(string(PetMedication::Name))
                    .col(date_null(PetMedication::NextMedicationDate))
                    .col(integer_null(PetMedication::FrequencyInterval))
                    .col(string_null(PetMedication::FrequencyUnit))
                    .col(integer(PetMedication::PetId))
                    .col(timestamp(PetMedication::Timestamp).default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pet-medication-medication-type-id")
                            .from(PetMedication::Table, PetMedication::MedicationTypeId)
                            .to(MedicationType::Table, MedicationType::MedicationTypeId)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pet-medication-pet-id")
                            .from(PetMedication::Table, PetMedication::PetId)
                            .to(Pet::Table, Pet::PetId)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(PetMedication::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PetMedication {
    Table,
    MedicationId,
    MedicationTypeId,
    Name,
    NextMedicationDate,
    FrequencyInterval,
    FrequencyUnit,
    PetId,
    Timestamp
}
