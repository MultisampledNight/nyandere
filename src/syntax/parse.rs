#![allow(clippy::double_parens)]

use chumsky::{Parser, input::Stream, prelude::*};
use logos::{Lexer, Logos};

use crate::ext::{Gtin, Money, Natural};

use super::{ast::*, lex::Token};

pub type Error<'src> = Rich<'src, Token<'src>, SimpleSpan>;
pub type Ctx<'src> = extra::Err<Error<'src>>;

impl<'src> Script<'src> {
    /// [`FromStr::from_str`] but not, since that doesn't allow lifetime constraints.
    pub fn parse(source: &'src str) -> ParseResult<Self, Error<'src>> {
        // based on https://github.com/zesterer/chumsky/blob/main/examples/logos.rs
        let iter = Token::lexer(source).spanned().map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(()) => (Token::Error, span.into()),
        });
        // navigatable by chumsky beyond just individual advancing
        let stream = Stream::from_iter(iter).map((0..SRC.len()).into(), |(t, s): (_, _)| (t, s));
        script().parse(stream)
    }
}

pub type TokenStream<'src> = Stream<Lexer<'src, (Token<'src>, SimpleSpan)>>;

/// Shorthand for the Parser trait.
pub trait P<'src, Node>: Parser<'src, TokenStream<'src>, Node, Ctx<'src>> {}
impl<'src, Node, T> P<'src, Node> for T where T: Parser<'src, TokenStream<'src>, Node, Ctx<'src>> {}

/// Shorthand for `P<'a, ()>`.
pub trait E<'a>: P<'a, ()> {}
impl<'a, T> E<'a> for T where T: P<'a, ()> {}

// complexity rises the further down
// read the return types as "what will *the parser returned by this function* return"
// not directly the function itself
//
// - when using `.or_not()` in a `cmd!`'s arg, chances are you actually want
//   to write `=> Parser::or_not` instead
//   - that'll optionalize the whitespace as well
//
// - for calming the lsp while writing new parsers, consider appending
//   `.ignore_then(todo())`

// basics

/// Hard/necessary inline whitespace.
fn hsp<'a>() -> impl E<'a> {
    todo!()
}

/// Optional inline whitespace.
fn osp<'a>() -> impl E<'a> {
    todo!()
}

pub fn ident<'a>() -> impl P<'a, Ident> {
    todo!()
}

pub fn nat<'a>() -> impl P<'a, Natural> {
    todo!()
}

/// Allows exactly 2 fractional digits,
/// returns them just left-shifted by 2 digits though
/// (as if the dot was not there).
pub fn decimal<'a>() -> impl P<'a, Natural> {
    todo!()
}

// literals

pub fn gtin<'a>() -> impl P<'a, Gtin> {
    todo!()
}

pub fn ratio<'a>() -> impl P<'a, Ratio> {
    todo!()
}

pub fn cents<'a>() -> impl P<'a, Money> {
    todo!()
}

pub fn euros<'a>() -> impl P<'a, Money> {
    todo!()
}

pub fn money<'a>() -> impl P<'a, Money> {
    todo!()
}

// commands

pub fn statement<'a>() -> impl P<'a, Stmt<'a>> {
    todo!()
}

// toplevel

pub fn delim<'a>() -> impl E<'a> {
    todo!()
}

pub fn script<'src>() -> impl P<'src, Script<'src>> {
    todo!()
}
