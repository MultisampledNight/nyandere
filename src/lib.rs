//! # Pipeline
//!
//! The general processing pipeline is:
//!
//! 1. Load source code as a string.
//! 2. Parse string into [`syntax::ast`] using [`syntax::parse`].
//! 3. Evaluate AST using [`model`].

#[macro_use]
extern crate macro_rules_attribute;

pub mod aux;
pub mod config;
pub mod ext;
pub mod model;
pub mod syntax;
pub mod ui;

use eyre::{Context, Result};
use syntax::parse;

pub fn run() -> Result<()> {
    let cfg = config::cli();
    let source = cfg.source.get().context("while loading source")?;
    dbg!(parse::parse(&source));

    Ok(())
}
