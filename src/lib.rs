pub mod entity;

use eyre::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, EntityTrait};
use time::macros::datetime;

#[tokio::main]
pub async fn run() -> Result<()> {
    let db = "sqlite:./track.db?mode=rwc";
    let db = Database::connect(db).await?;

    Migrator::up(&db, None).await?;

    let purchase = entity::product::ActiveModel {
        name: ActiveValue::set(Some("mate".to_string())),
        default_price: ActiveValue::set(500),
        created_at: ActiveValue::set(datetime!(2024-10-03 05:07)),
        ..Default::default()
    };

    purchase.insert(&db).await?;

    dbg!(entity::product::Entity::find().all(&db).await?);

    Ok(())
}
