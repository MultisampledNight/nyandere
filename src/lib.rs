//! # Pipeline
//!
//! The general processing pipeline is:
//!
//! 1. Load source code as a string.
//! 2. Parse string into a [`Script`] using [`syntax::parse`].
//!     - [`Script`] serves as the AST root
//! 3. Run the [`Script`] in the [`Runtime`] using [`Runtime::run`]
//!
//! # Adding new commands
//!
//! Task list for adding a new command `nya`:
//!
//! 1. Syntax
//!    a. Think of one (where is it used? what are its arguments?)
//!    b. Update `doc/syntax.abnf` appropriately
//!    c. Expand the AST in [`syntax::ast`]
//!    to include `nya` below [`Stmt`][syntax::ast::Stmt]
//!    in the tree
//!    d. Parse it in [`syntax::parse`] (the `cmd!` macro is super useful here!)
//! 2. Logic
//!    a. Add a submodule `nya` in [`runtime::cmd`] for the command
//!    b. In there, write a type `Nya` with the type-restricted arguments for the command
//!    c. Enforce the semantic requirements between its AST and the command in [`runtime::repr`]
//!    d. Write a method on [`Runtime`] that performs the op
//!    e. Dispatch it in [`runtime::cmd`]
//! 3. All done! Test, test, test!

#[macro_use]
extern crate macro_rules_attribute;

pub mod aux;
pub mod ext;
pub mod runtime;
pub mod syntax;

use ext::config;
pub use runtime::Runtime;
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
