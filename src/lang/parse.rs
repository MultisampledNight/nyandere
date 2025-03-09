#![allow(unused)]

use chumsky::{
    Parser,
    prelude::*,
    text::{inline_whitespace, int, keyword, newline},
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
                param!($arg_1 $(=> $arg_1_post)?),
                $(param!($arg_n $(=> $arg_n_post)?)),*
            )))
        )?
    };
}

/// Prepends a hard space before the given argument,
/// optionally with post-processing.
macro_rules! param {
    ($pre:expr $(=> $post:expr)?) => {
        $(({ $post }))? (hsp().ignore_then({ $pre }))
    };
}

/// Returns the contained element for a single-element tuple.
fn untup<T>((ele,): (T,)) -> T {
    ele
}

/// Shorthand for the Parser trait.
pub trait P<'a, Node>: Parser<'a, &'a str, Node, Ctx<'a>> {}
impl<'a, Node, T> P<'a, Node> for T where T: Parser<'a, &'a str, Node, Ctx<'a>> {}

/// Shorthand for `P<'a, ()>`.
trait E<'a>: P<'a, ()> {}
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
    inline_whitespace().at_least(1)
}

/// Optional inline whitespace.
fn osp<'a>() -> impl E<'a> {
    inline_whitespace()
}

fn ident<'a>() -> impl P<'a, Ident> {
    chumsky::text::ident().map(Ident::new)
}

fn nat<'a>() -> impl P<'a, Natural> {
    int(10)
        .map(str::parse)
        // expecting the int parser to only accept valid ints
        .unwrapped()
}

// literals

fn gtin<'a>() -> impl P<'a, Gtin> {
    todo()
}

fn cents<'a>() -> impl P<'a, Money> {
    nat()
        .then_ignore(
            osp()
                .then(choice((just("cents"), just("ct"), just("¢"))))
                .or_not(),
        )
        .map(Money)
}

fn euros<'a>() -> impl P<'a, Money> {
    todo()
}

fn money<'a>() -> impl P<'a, Money> {
    choice((cents(), euros()))
}

// parameters

fn from<'a>() -> impl P<'a, Ident> {
    cmd!("from" ident()).map(untup)
}

fn to<'a>() -> impl P<'a, Ident> {
    cmd!("to" ident()).map(untup)
}

fn dir<'a>() -> impl P<'a, Dir> {
    group((from(), hsp(), to())).map(|(from, _, to)| Dir { from, to })
}

fn product<'a>() -> impl P<'a, Product> {
    choice((ident().map(Product::Name), gtin().map(Product::Id)))
}

fn value<'a>() -> impl P<'a, Money> {
    money()
}

fn price<'a>() -> impl P<'a, Money> {
    cmd!("price" money()).map(untup)
}

// actors

fn entity<'a>() -> impl P<'a, Entity> {
    cmd!("entity" ident()).map(|(name,)| Entity { name })
}

fn object<'a>() -> impl P<'a, Object> {
    cmd!(
        "object"
        ident(),
        cmd!("parent" ident()).map(untup) => Parser::or_not,
    )
    .map(|(name, parent)| Object { name, parent })
}

fn concept<'a>() -> impl P<'a, Concept> {
    cmd!(
        "concept"
        ident(),
        price() => Parser::or_not,
        cmd!("gtin" gtin()).map(untup) => Parser::or_not,
    )
    .map(|(name, default_price, gtin)| Concept {
        name,
        default_price,
        gtin,
    })
}

fn actor<'a>() -> impl P<'a, Actor> {
    choice((
        entity().map(Actor::Entity),
        object().map(Actor::Object),
        concept().map(Actor::Concept),
    ))
}

// commands

fn create<'a>() -> impl P<'a, Create> {
    cmd!("create" actor()).map(|(who,)| Create { who })
}

fn pay<'a>() -> impl P<'a, Pay> {
    cmd!("pay" value(), dir()).map(|(amount, who)| Pay { amount, who })
}

fn deliver<'a>() -> impl P<'a, Deliver> {
    cmd!(
        "deliver"
        product(),
        price() => Parser::or_not,
        dir(),
    )
    .map(|(what, price, who)| Deliver { what, price, who })
}

fn purchase<'a>() -> impl P<'a, Purchase> {
    cmd!(
        "purchase"
        product(),
        price() => Parser::or_not,
        dir(),
    )
    .map(|(what, price, who)| Purchase { what, price, who })
}

fn stats<'a>() -> impl P<'a, Stats> {
    choice((keyword("stats"), keyword("statistics"))).map(|_| Stats)
}

fn balance<'a>() -> impl P<'a, Balance> {
    choice((keyword("balance"), keyword("bal")))
        .ignore_then(dir())
        .map(|between| Balance { between })
}

fn transfer<'a>() -> impl P<'a, Transfer> {
    choice((
        pay().map(Transfer::Pay),
        deliver().map(Transfer::Deliver),
        purchase().map(Transfer::Purchase),
    ))
}

fn analyze<'a>() -> impl P<'a, Analyze> {
    choice((stats().map(Analyze::Stats), balance().map(Analyze::Balance)))
}

fn statement<'a>() -> impl P<'a, Stmt> {
    choice((
        create().map(Stmt::Create),
        transfer().map(Stmt::Transfer),
        analyze().map(Stmt::Analyze),
    ))
}

// toplevel

/// Upon a `#`, ignore everything until end of line or end of input.
fn comment<'a>() -> impl E<'a> {
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

fn delim<'a>() -> impl E<'a> {
    choice((newline(), just(';').ignored())).padded()
}

pub fn script<'a>() -> impl P<'a, Script> {
    statement()
        .padded_by(osp())
        .separated_by(choice((delim(), comment())))
        .allow_leading()
        .allow_trailing()
        .collect()
        .map(Script)
}
