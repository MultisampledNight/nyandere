use chumsky::{
    Parser,
    prelude::*,
    text::{inline_whitespace, keyword, newline},
};

use crate::ext::Gtin;

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
    ($name:literal $( $arg_1:expr $(, $arg_n:expr)* $(,)? )? ) => {
        keyword($name)$(
            .ignore_then(group((
                hsp().ignore_then($arg_1),
                $(hsp().ignore_then($arg_n)),*
            )))
        )?
    };
}

macro_rules! parser {
    ($(
        $( #[$attr:meta] )*
        $vis:vis fn $fn_name:ident
        ($($param_name:ident : $param_type:ty),* $(,)?)
        -> $ret:ty
        { $body:expr }
    )*) => {$(
        $( #[$attr] )*
        fn $fn_name<'a>($( $param_name : $param_type ),*)
            -> impl Parser<'a, &'a str, $ret, Ctx<'a>>
        {
            $body
        }
    )*};
}

/// Returns the contained element for a single-element tuple.
fn untup<T>((ele,): (T,)) -> T {
    ele
}

// complexity rises the further down
// read the return types as "what will *the parser returned by this function* return"
// not directly the function itself
//
// for calming the lsp while writing new parsers, consider appending
// `.ignore_then(todo())`
parser! {
    /// Hard/necessary inline whitespace.
    fn hsp() -> () {
        inline_whitespace().at_least(1)
    }

    fn ident() -> Ident {
        chumsky::text::ident().map(Ident::new)
    }

    fn gtin() -> Gtin {
        todo()
    }

    fn money() -> Money {
        todo()
    }

    fn price() -> Money {
        cmd!("price" money()).map(untup)
    }

    fn entity() -> Entity {
        cmd!(
            "entity"
            ident()
        ).map(|(name,)| Entity { name })
    }

    fn object() -> Object {
        cmd!(
            "object"
            ident(),
            // TODO: doesn't this need a space after the ident even if this is the `not` case?
            cmd!("instance-of" ident()).map(untup).or_not(),
        ).map(|(name, instance_of)| Object { name, instance_of })
    }

    fn concept() -> Concept {
        cmd!(
            "concept"
            ident(),
            price().or_not(),
            cmd!("gtin" gtin()).map(untup).or_not(),
        ).ignore_then(todo())
    }

    fn actor() -> Actor {
        choice((
                entity().map(Actor::Entity),
                object().map(Actor::Object),
                concept().map(Actor::Concept),
        ))
    }

    fn create() -> Create {
        cmd!("create" actor()).map(|(who,)| Create { who })
    }

    fn statement() -> Stmt {
        choice((create().map(Stmt::Create),))
    }

    /// Upon a `#`, ignore everything until end of line or end of input.
    fn comment() -> () {
        just('#')
            // What can appear in a comment?
            // Lazy since `repeated` is greedy by default
            // (would cause comments to include the next lines as well)
            .then(any().repeated().lazy())
            // How can comments be ended?
            .then(choice((newline(), end())))
            // Not modeled in the AST.
            .ignored()
    }

    fn delim() -> () {
        choice((newline(), just(';').ignored())).padded()
    }

    pub fn script() -> Script {
        statement()
            .separated_by(choice((delim(), comment())))
            .allow_leading()
            .allow_trailing()
            .collect()
            .map(Script)
    }
}
