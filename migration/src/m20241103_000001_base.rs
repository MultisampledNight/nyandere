use sea_orm_migration::prelude::*;

use crate::id;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        module_path!().split("::").last().unwrap()
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .col(id(Product::Id))
                    .col(ColumnDef::new(Product::Gtin).integer().unique_key().null())
                    .col(ColumnDef::new(Product::Name).string().unique_key().null())
                    .col(ColumnDef::new(Product::DefaultPrice).integer().not_null())
                    .col(ColumnDef::new(Product::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .col(id(Session::Id))
                    .col(ColumnDef::new(Session::StartedAt).date_time().not_null())
                    .col(ColumnDef::new(Session::CompletedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Purchase::Table)
                    .col(id(Purchase::Id))
                    .col(ColumnDef::new(Purchase::Of).integer().not_null())
                    .col(ColumnDef::new(Purchase::During).integer().not_null())
                    .col(count(Purchase::Count))
                    .col(ColumnDef::new(Purchase::TotalPrice).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_purchase_product")
                            .from(Purchase::Table, Purchase::Of)
                            .to(Product::Table, Product::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_purchase_session")
                            .from(Purchase::Table, Purchase::During)
                            .to(Session::Table, Session::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Consumption::Table)
                    .col(id(Consumption::Id))
                    .col(ColumnDef::new(Consumption::Of).integer().not_null())
                    .col(ColumnDef::new(Consumption::At).date_time().not_null())
                    .col(count(Consumption::Count))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_consumption_product")
                            .from(Consumption::Table, Consumption::Of)
                            .to(Product::Table, Product::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for who in [
            Product::Table.into_iden(),
            Session::Table.into_iden(),
            Purchase::Table.into_iden(),
            Consumption::Table.into_iden(),
        ] {
            manager
                .drop_table(Table::drop().table(who).to_owned())
                .await?;
        }
        Ok(())
    }
}

#[derive(Iden)]
pub enum Product {
    Table,
    Id,
    Gtin,
    Name,
    DefaultPrice,
    CreatedAt,
}

#[derive(Iden)]
pub enum Session {
    Table,
    Id,
    StartedAt,
    CompletedAt,
}

#[derive(Iden)]
pub enum Purchase {
    Table,
    Id,
    Of,
    During,
    Count,
    TotalPrice,
}

#[derive(Iden)]
pub enum Consumption {
    Table,
    Id,
    Of,
    At,
    Count,
}

fn count(iden: impl IntoIden) -> ColumnDef {
    let iden = iden.into_iden();
    ColumnDef::new(iden.clone())
        .unsigned()
        .not_null()
        .default(1u32)
        .check(Expr::col(iden).gt(0))
        .take()
}
