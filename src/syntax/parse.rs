#![allow(clippy::double_parens)]

use chumsky::{
    Parser,
    input::{Stream, ValueInput},
    prelude::*,
};
use logos::Logos;

use super::{ast::*, lex::Token};

pub type Error<'tok, 'src> = Rich<'tok, Token<'src>, SimpleSpan>;
pub type Ctx<'tok, 'src> = extra::Err<Error<'tok, 'src>>;

impl<'tok> Script<'tok> {
    /// [`FromStr::from_str`] but not, since that doesn't allow lifetime constraints.
    pub fn parse<'src: 'tok>(source: &'src str) -> ParseResult<Self, Error<'tok, 'src>> {
        // based on https://github.com/zesterer/chumsky/blob/main/examples/logos.rs
        let iter = Token::lexer(source).spanned().map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(()) => (Token::Error, span.into()),
        });

        // used for EOF tokens
        let end_span = (source.len()..source.len()).into();

        // navigatable by chumsky beyond just individual advancing
        let stream = Stream::from_iter(iter).map(end_span, |(t, s): (_, _)| (t, s));
        parser().parse(stream)
    }
}

pub fn parser<'tok, 'src: 'tok, I>() -> impl Parser<'tok, I, Script<'tok>, Ctx<'tok, 'src>>
where
    I: ValueInput<'tok, Token = Token<'src>, Span = SimpleSpan>,
{
    todo()
}
