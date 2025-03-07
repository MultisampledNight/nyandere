pub mod config;
pub mod model;
pub mod ui;

use eyre::Result;

#[tokio::main]
pub async fn run() -> Result<()> {
    let _cfg = config::cli();

    Ok(())
}
