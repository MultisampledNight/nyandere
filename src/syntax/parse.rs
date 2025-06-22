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
        let iter = Token::lexer(source)
            .spanned()
            .map(|x| dbg!(x))
            .map(|(tok, span)| match tok {
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
    use Token as T;

    let optional_space = just(T::Whitespace).repeated();
    //let hard_space = just(T::Whitespace).repeated().at_least(1);

    let statement_delimiter = one_of([T::Semicolon, T::Newline]).padded_by(optional_space);

    let command = todo();
    let arguments = todo();

    let statement = group((command, arguments)).map(|(cmd, args)| Stmt { cmd, args });

    let script = statement
        .separated_by(statement_delimiter.repeated().at_least(1))
        .allow_leading()
        .allow_trailing()
        .collect::<Vec<_>>()
        .map(Script);

    script
}
