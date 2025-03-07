use chumsky::{
    Parser,
    prelude::*,
    text::{ident, inline_whitespace, keyword, newline},
};

use super::ast::*;

pub type Error<'a> = Rich<'a, char, SimpleSpan>;
pub type Ctx<'a> = extra::Err<Error<'a>>;

pub fn parse<'a>(src: &'a str) -> ParseResult<Script, Error<'a>> {
    script().parse(src)
}

pub fn script<'a>() -> impl Parser<'a, &'a str, Script, Ctx<'a>> {
    let kw = |name| keyword(name).ignored();

    // Hard/necessary inline whitespace
    let hsp = inline_whitespace().at_least(1);

    let entity = group((kw("entity"), hsp, ident())).map(|(_, _, name): (_, _, &str)| Entity {
        name: Ident::new(name.to_string()),
    });
    let object = todo();
    let concept = todo();

    let actor = choice((
        entity.map(Actor::Entity),
        object.map(Actor::Object),
        concept.map(Actor::Concept),
    ));

    let create = group((kw("create"), hsp, actor)).map(|(_, _, who)| Create { who });
    let statement = choice((create.map(Stmt::Create), todo()));

    // Upon a `#`, ignore everything
    let comment = just('#')
        // What can appear in a comment?
        // Lazy since `repeated` is greedy by default
        // (would cause comments to include the next lines as well)
        .then(any().repeated().lazy())
        // How can comments be ended?
        .then(choice((newline(), end())))
        // Not modeled in the AST.
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
