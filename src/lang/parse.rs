use chumsky::{Parser, prelude::*, text::newline};

use super::ast::Script;

pub type Error<'a> = Rich<'a, char, SimpleSpan>;
pub type Ctx<'a> = extra::Err<Error<'a>>;

pub fn parse<'a>(src: &'a str) -> ParseResult<Script, Error<'a>> {
    script().parse(src)
}

pub fn script<'a>() -> impl Parser<'a, &'a str, Script, Ctx<'a>> {
    let statement = todo();

    // upon a `#`, ignore everything
    // until a newline or end of input
    let comment = just('#')
        .then(any().repeated().lazy())
        .then(choice((newline(), end())))
        .ignored();

    // statement delimiters
    let delim = choice((newline(), just(';').ignored())).padded();

    statement
        .separated_by(choice((delim, comment)))
        .allow_leading()
        .allow_trailing()
        .collect()
        .map(Script)
}
