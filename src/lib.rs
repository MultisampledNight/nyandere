#[macro_use]
extern crate macro_rules_attribute;

pub mod aux;
pub mod config;
pub mod ext;
pub mod lang;
pub mod model;
pub mod ui;

use eyre::{Context, Result};
use lang::parse;

pub fn run() -> Result<()> {
    let cfg = config::cli();
    let source = cfg.source.get().context("while loading source")?;
    dbg!(parse::parse(&source));

    Ok(())
}
