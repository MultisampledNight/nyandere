//! # Pipeline
//!
//! The general processing pipeline is:
//!
//! 1. Load source code as a string.
//! 2. Parse string into a [`Script`] using [`syntax::parse`].
//!     - [`Script`] serves as the AST root
//! 3. Run the [`Script`] in the [`Runtime`] using [`Runtime::run`]

#[macro_use]
extern crate macro_rules_attribute;

pub mod aux;
pub mod config;
pub mod ext;
pub mod model;
pub mod syntax;
pub mod ui;

pub use model::runtime::Runtime;
pub use syntax::ast::Script;

use eyre::{Result, WrapErr, format_err};

pub fn run() -> Result<()> {
    let cfg = config::cli();

    let script = cfg.source.get().wrap_err("while loading source")?;
    // TODO: throw the error into ariadne to render with better UX
    let script = Script::parse(&script)
        .into_result()
        .map_err(|orig| format_err!("while parsing source code: {orig:?}"))?;

    let mut runtime = Runtime::new();
    runtime.run(script).wrap_err("while evaluating")?;
    dbg!(runtime);

    Ok(())
}
