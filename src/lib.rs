pub mod config;
pub mod model;
pub mod store;

use eyre::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

#[tokio::main]
pub async fn run() -> Result<()> {
    let db = "sqlite:./track.db?mode=rwc";
    let db = Database::connect(db).await?;

    Migrator::up(&db, None).await?;

    Ok(())
}
