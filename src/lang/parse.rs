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

/// Takes a command discriminant before the parens
/// and arguments in the parens, returning a parser for it.
/// *n* arguments lead to the return type of `(T_1, ..., T_n)`.
macro_rules! cmd {
    // split of 1 vs n is to avoid putting choice at all if there are any arguments
    ($name:ident ($( $arg_1:expr $(, $arg_n:expr)* $(,)? )?) ) => {
        keyword(stringify!($name))$(
            .ignore_then(group((
                hsp().ignore_then($arg_1),
                $(hsp().ignore_then($arg_n)),*
            )))
        )?
    };
}

macro_rules! shorthand {
    ($(
        $( #[$attr:meta] )*
        $fn_name:ident
        ($($param_name:ident : $param_type:ty),* $(,)?)
        -> $ret:ty
        = $body:expr
    );* $(;)?) => {$(
        $( #[$attr] )*
        fn $fn_name<'a>($( $param_name : $param_type ),*)
            -> impl Parser<'a, &'a str, $ret, Ctx<'a>>
        {
            ($body)()
        }
    )*};
}

shorthand! {
    /// Hard/necessary inline whitespace.
    hsp() -> () = || inline_whitespace().at_least(1);
}

pub fn script<'a>() -> impl Parser<'a, &'a str, Script, Ctx<'a>> {
    let entity = cmd!(entity(ident())).map(|(name,)| Entity {
        name: Ident::new(name.to_string()),
    });
    let object = todo();
    let concept = todo();

    let actor = choice((
        entity.map(Actor::Entity),
        object.map(Actor::Object),
        concept.map(Actor::Concept),
    ));

    let create = cmd!(create(actor)).map(|(who,)| Create { who });
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
