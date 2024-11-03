pub use sea_orm_migration::prelude::*;

mod m20241103_000001_base;

pub struct Migrator;

pub fn id(iden: impl IntoIden) -> ColumnDef {
    ColumnDef::new(iden)
        .integer()
        .not_null()
        .auto_increment()
        .primary_key()
        .take()
}

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241103_000001_base::Migration)]
    }
}
