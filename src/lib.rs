#[macro_use]
extern crate macro_rules_attribute;

pub mod aux;
pub mod config;
pub mod ext;
pub mod lang;
pub mod model;

use eyre::Result;
use lang::parse;

pub fn run() -> Result<()> {
    let cfg = config::cli();
    dbg!(parse::parse(&cfg.code));

    Ok(())
}
