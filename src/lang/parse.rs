#![allow(unused)]

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
    (
        $name:literal
        $(
            $arg_1:expr $(=> $arg_1_post:expr)?
            $(, $arg_n:expr $(=> $arg_n_post:expr)? )* $(,)?
        )?
    ) => {
        keyword($name)$(
            .ignore_then(group((
                $(($arg_1_post))? (hsp().ignore_then($arg_1)),
                $($(($arg_n_post))? (hsp().ignore_then($arg_n))),*
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
// - when using `.or_not()` in a `cmd!`'s arg, chances are you actually want
//   to write `=> Parser::or_not` instead
//   - that'll optionalize the whitespace as well
//
// - for calming the lsp while writing new parsers, consider appending
//   `.ignore_then(todo())`
parser! {
    // basics

    /// Hard/necessary inline whitespace.
    fn hsp() -> () {
        inline_whitespace().at_least(1)
    }

    fn ident() -> Ident {
        chumsky::text::ident().map(Ident::new)
    }

    // literals

    fn gtin() -> Gtin {
        todo()
    }

    fn cents() -> Money {
        todo()
    }

    fn euros() -> Money {
        todo()
    }

    fn money() -> Money {
        todo()
    }

    // parameters
    
    fn from() -> Ident {
        todo()
    }

    fn to() -> Ident {
        todo()
    }

    fn dir() -> Dir {
        todo()
    }

    fn product() -> Product {
        todo()
    }

    fn value() -> Money {
        todo()
    }

    fn price() -> Money {
        cmd!("price" money()).map(untup)
    }

    // actors

    fn entity() -> Entity {
        cmd!(
            "entity"
            ident()
        )
        .map(|(name,)| Entity { name })
    }

    fn object() -> Object {
        cmd!(
            "object"
            ident(),
            cmd!("parent" ident()).map(untup) => Parser::or_not,
        )
        .map(|(name, parent)| Object { name, parent })
    }

    fn concept() -> Concept {
        cmd!(
            "concept"
            ident(),
            price().or_not(),
            cmd!("gtin" gtin()).map(untup) => Parser::or_not,
        )
        .ignore_then(todo())
    }

    fn actor() -> Actor {
        choice((
            entity().map(Actor::Entity),
            object().map(Actor::Object),
            concept().map(Actor::Concept),
        ))
    }

    // commands

    fn create() -> Create {
        cmd!("create" actor()).map(|(who,)| Create { who })
    }

    fn pay() -> Pay {
        todo()
    }

    fn deliver() -> Deliver {
        todo()
    }

    fn purchase() -> Purchase {
        todo()
    }

    fn stats() -> Stats {
        todo()
    }

    fn balance() -> Balance {
        todo()
    }

    fn transfer() -> Transfer {
        choice((
            pay().map(Transfer::Pay),
            deliver().map(Transfer::Deliver),
            purchase().map(Transfer::Purchase),
        ))
    }

    fn analyze() -> Analyze {
        choice((
            stats().map(Analyze::Stats),
            balance().map(Analyze::Balance),
        ))
    }

    fn statement() -> Stmt {
        choice((
            create().map(Stmt::Create),
            transfer().map(Stmt::Transfer),
            analyze().map(Stmt::Analyze),
        ))
    }

    // toplevel

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
            .padded_by(inline_whitespace())
            .separated_by(choice((delim(), comment())))
            .allow_leading()
            .allow_trailing()
            .collect()
            .map(Script)
    }
}
