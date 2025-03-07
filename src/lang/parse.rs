use chumsky::prelude::*;

use super::ast::Script;

/// Alias for [`chumsky::Parser`] so we don't need to write out the input type all the time.
pub trait P<'a, Node>: chumsky::Parser<'a, &'a str, Node> {}
impl<'a, Node, T: chumsky::Parser<'a, &'a str, Node>> P<'a, Node> for T {}

pub fn script<'a>() -> impl P<'a, Script> {
    todo()
}
