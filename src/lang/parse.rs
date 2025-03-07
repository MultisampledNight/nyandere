use chumsky::{Parser, prelude::*, text::newline};

use super::ast::Script;

pub type Error<'a> = Rich<'a, char, SimpleSpan>;
pub type Ctx<'a> = extra::Err<Error<'a>>;

macro_rules! eval {
    ($parser:expr => $input:expr) => {
        let res: ParseResult<_, Error> = $parser.parse($input);
        dbg!(res);
    };
}

pub fn parse<'a>(src: &'a str) -> ParseResult<Script, Error<'a>> {
    // upon a `#`, ignore everything
    // until a newline or end of input
    let comment = just::<_, _, Ctx>('#')
        .then(any().repeated().lazy())
        .then(choice((newline(), end())))
        .ignored();

    eval!(comment => src);

    todo::<_, _, Ctx>().parse(src)
}
