pub mod config;
pub mod ext;
pub mod lang;
pub mod model;

use eyre::Result;

pub fn run() -> Result<()> {
    let _cfg = config::cli();

    Ok(())
}
