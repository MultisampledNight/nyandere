pub mod config;
pub mod model;
pub mod ui;

use eyre::Result;

pub fn run() -> Result<()> {
    let _cfg = config::cli();

    Ok(())
}
