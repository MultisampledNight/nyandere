use eyre::Result;
use sea_orm::Database;

#[tokio::main]
pub async fn run() -> Result<()> {
    let db = "sqlite:./track.db?mode=rwc";
    let db = Database::connect(db).await?;

    Ok(())
}
