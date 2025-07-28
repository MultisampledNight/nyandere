#![allow(clippy::double_parens)]

use chumsky::{
    Parser,
    input::{Stream, ValueInput},
    prelude::*,
};
use logos::Logos;

use crate::ext::{Money, Natural};

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

/// Invokes the output type's `FromStr` impl. If it errors, that is treated like a parsing error
/// and its message is directly displayed.
macro_rules! from_str {
    ($parser:expr) => {
        ($parser).try_map(|src, span| src.parse().map_err(|err| Error::custom(span, err)))
    };
}

pub fn parser<'tok, 'src: 'tok, I>() -> impl Parser<'tok, I, Script<'tok>, Ctx<'tok, 'src>>
where
    I: ValueInput<'tok, Token = Token<'src>, Span = SimpleSpan>,
{
    use Token as T;

    const DOT_SHIFT: u8 = 10u8.pow(2);

    let optional_space = just(T::Whitespace).repeated();
    let hard_space = optional_space.at_least(1);

    let statement_delimiter = one_of([T::Semicolon, T::Newline]).padded_by(optional_space);

    let ident = select! { T::Ident(id) => Ident(id) };

    // pretty much just faking and directly converting into an integer
    // since we know there's only 2 digits after the dot
    let decimal = select! { T::Decimal(src) => src }.map(|src| {
        let (whole, fraction) = src
            .rsplit_once('.')
            .expect("lexer to emit decimal token only with a dot");

        let nat = |src: &str| src.parse::<Natural>().unwrap();

        nat(whole) * DOT_SHIFT + nat(fraction)
    });
    // expectation: lexer already made sure that this is, in fact, a natural number
    let natural = select! { T::Natural(src) => src }.from_str().unwrapped();

    let cents = natural.then_ignore(optional_space.then(just(T::SignCent)));
    let euros = choice((decimal, natural.map(|num| num * DOT_SHIFT)))
        .then_ignore(optional_space.then(just(T::SignEuro)));
    let money = choice((euros, cents)).map(Money);

    let split = group((natural, just(T::Colon).padded_by(optional_space), natural))
        .map(|(from, _, to)| Split { from, to });

    let gtin = from_str!(select! { T::Natural(src) => src });

    let name = ident.map(Name);

    let value = choice((
        money.map(Value::Money),
        split.map(Value::Split),
        gtin.map(Value::Gtin),
        name.map(Value::Name),
    ));

    let named = group((ident, just(T::Equals).padded_by(optional_space), value))
        .map(|(key, _, value)| Arg::Named { key, value });
    let positional = value.map(Arg::Pos);

    let arguments = choice((named, positional))
        .separated_by(hard_space)
        .collect::<Vec<_>>()
        .map(Args);

    let command = select! {
        T::Create => Command::Create,
        T::Pay => Command::Pay,
        T::Deliver => Command::Deliver,
        T::Balance => Command::Balance,
    };

    let statement =
        group((command, hard_space, arguments)).map(|(cmd, _, args)| Stmt { cmd, args });

    statement
        .separated_by(statement_delimiter.repeated().at_least(1))
        .allow_leading()
        .allow_trailing()
        .collect::<Vec<_>>()
        .map(Script)
}
