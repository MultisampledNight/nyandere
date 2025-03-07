pub mod config;
pub mod ext;
pub mod lang;
pub mod model;

use eyre::Result;
use lang::parse;

pub fn run() -> Result<()> {
    let cfg = config::cli();
    parse::parse(&cfg.code);

    Ok(())
}
