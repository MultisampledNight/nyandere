//! An [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree).
//!
//! First layer of abstraction over the source text.

use crate::{
    aux::Owned,
    ext::{Gtin, Money, Natural},
};

#[derive(Owned!)]
pub struct Script(pub Vec<Stmt>);

/// Something that can be done.
#[derive(Owned!)]
pub struct Stmt {
    pub command: Ident,
    pub args: Args,
}

#[derive(Owned!)]
pub struct Args {
    pub positional: Vec<Expr>,
    pub key_value: Vec<(Ident, Expr)>,
}

#[derive(Owned!)]
pub enum Expr {
    Lit(Lit),
    Var(Ident),
}

#[derive(Owned!)]
pub enum Lit {
    Money(Money),
    Ratio(Ratio),
    Gtin(Gtin),
}

/// Distribution between left and right.
#[derive(Owned!)]
pub struct Ratio {
    pub left: Natural,
    pub right: Natural,
}

#[derive(Owned!)]
pub struct Ident(String);

impl Ident {
    /// Use for parsing only.
    pub(super) fn new(ident: impl AsRef<str>) -> Self {
        Self(ident.as_ref().to_string())
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl From<Ident> for String {
    fn from(id: Ident) -> Self {
        id.0
    }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
