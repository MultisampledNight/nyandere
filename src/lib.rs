pub mod config;
pub mod model;

use eyre::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

#[tokio::main]
pub async fn run() -> Result<()> {
    let cfg = config::cli();
    let db = Database::connect(cfg.database).await?;

    Migrator::up(&db, None).await?;

    Ok(())
}
