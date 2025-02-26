pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_pet_table;
mod m20220101_000001_create_pet_photo_table;
mod m20220101_000001_create_medication_type_table;
mod m20220101_000001_create_pet_medication_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_pet_table::Migration),
             Box::new(m20220101_000001_create_medication_type_table::Migration),
             Box::new(m20220101_000001_create_pet_medication_table::Migration),
             Box::new(m20220101_000001_create_pet_photo_table::Migration)]
    }
}
