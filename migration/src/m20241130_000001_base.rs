use sea_orm_migration::prelude::*;

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
                    .table(Concept::Table)
                    .col(id(Concept::Id))
                    .col(fk(Concept::During))
                    .col(def(Concept::Gtin).integer().unique_key().null())
                    .col(def(Concept::Name).string().unique_key().null())
                    .col(def(Concept::DefaultPrice).integer().null())
                    .foreign_key(&mut session(Concept::Table, Concept::During))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .col(id(Session::Id))
                    .col(at(Session::Start))
                    .col(at(Session::End))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(id(User::Id))
                    .col(def(User::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Object::Table)
                    .col(id(Object::Id))
                    .col(fk(Object::InstanceOf))
                    .foreign_key(&mut concept(Object::Table, Object::InstanceOf))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Delivery::Table)
                    .col(id(Delivery::Id))
                    .col(fk(Delivery::During))
                    .col(fk(Delivery::Of))
                    .col(fk(Delivery::Payment))
                    .col(at(Delivery::At))
                    .foreign_key(&mut session(Delivery::Table, Delivery::During))
                    .foreign_key(&mut object(Delivery::Table, Delivery::Of))
                    .foreign_key(&mut payment(Delivery::Table, Delivery::Payment))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Payment::Table)
                    .col(id(Payment::Id))
                    .col(fk(Payment::During))
                    .col(fk(Payment::From))
                    .col(fk(Payment::To))
                    .col(posint(Payment::Amount))
                    .col(at(Payment::At))
                    .foreign_key(&mut session(Payment::Table, Payment::During))
                    .foreign_key(&mut user(Payment::Table, Payment::From))
                    .foreign_key(&mut user(Payment::Table, Payment::To))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for who in [
            Concept::Table.into_iden(),
            Session::Table.into_iden(),
            User::Table.into_iden(),
            Object::Table.into_iden(),
            Delivery::Table.into_iden(),
            Payment::Table.into_iden(),
        ] {
            manager
                .drop_table(Table::drop().table(who).to_owned())
                .await?;
        }
        Ok(())
    }
}

#[derive(Iden)]
pub enum Concept {
    Table,
    Id,
    During,
    Gtin,
    Name,
    DefaultPrice,
}

#[derive(Iden)]
pub enum Session {
    Table,
    Id,
    Start,
    End,
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum Object {
    Table,
    Id,
    InstanceOf,
}

#[derive(Iden)]
pub enum Delivery {
    Table,
    Id,
    During,
    Of,
    Payment,
    At,
}

#[derive(Iden)]
pub enum Payment {
    Table,
    Id,
    During,
    From,
    To,
    Amount,
    At,
}

pub fn at(iden: impl IntoIden) -> ColumnDef {
    def(iden).date_time().not_null().take()
}

fn posint(iden: impl IntoIden) -> ColumnDef {
    let iden = iden.into_iden();
    def(iden.clone())
        .unsigned()
        .not_null()
        // not 100% sure if this is necessary. but i think it might be as sqlite has no concept of
        // "unsigned", only integer
        .check(Expr::col(iden).gte(0))
        .take()
}

macro_rules! fk_shorthands {
    ($( $name:ident: $to_table:expr => $to_col:expr ),* $(,)?) => {$(
        pub fn $name(from_table: impl IntoIden, from_col: impl IntoIden) -> ForeignKeyCreateStatement {
            fk_rel(from_table, from_col, $to_table, $to_col)
        }
    )*};
}

fk_shorthands! {
    concept: Concept::Table => Concept::Id,
    session: Session::Table => Session::Id,
    user: User::Table => User::Id,
    object: Object::Table => Object::Id,
    payment: Payment::Table => Payment::Id,
}

pub fn fk_rel(
    from_table: impl IntoIden,
    from_col: impl IntoIden,
    to_table: impl IntoIden,
    to_col: impl IntoIden,
) -> ForeignKeyCreateStatement {
    let from_table = from_table.into_iden();
    let to_table = to_table.into_iden();
    ForeignKey::create()
        .name(format!(
            "fk_{}_{}",
            from_table.to_string(),
            to_table.to_string()
        ))
        .from(from_table, from_col)
        .to(to_table, to_col)
        .take()
}

pub fn fk(iden: impl IntoIden) -> ColumnDef {
    def(iden).integer().not_null().take()
}

pub fn id(iden: impl IntoIden) -> ColumnDef {
    def(iden)
        .integer()
        .not_null()
        .auto_increment()
        .primary_key()
        .take()
}

pub fn def(iden: impl IntoIden) -> ColumnDef {
    ColumnDef::new(iden)
}
