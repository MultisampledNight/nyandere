#![allow(clippy::double_parens)]

use chumsky::{
    Parser,
    prelude::*,
    text::{digits, inline_whitespace, int, keyword, newline},
};

use crate::ext::{Gtin, Money, Natural};

use super::ast::*;

pub type Error<'a> = Rich<'a, char, SimpleSpan>;
pub type Ctx<'a> = extra::Err<Error<'a>>;

impl Script {
    /// [`FromStr::from_str`] but not, since that doesn't allow lifetime restraints.
    pub fn parse<'text>(source: &'text str) -> ParseResult<Self, Error<'text>> {
        script().parse(source)
    }
}

/// Takes a command discriminant before the parens
/// and arguments in the parens, returning a parser for it.
/// *n* arguments lead to the return type of `(T_1, ..., T_n)`.
macro_rules! cmd {
    // split of 1 vs n is to avoid putting choice at all if there are no arguments
    (
        $($name:literal)|+ $( :
            $arg_1:expr $(=> $arg_1_post:expr)?
            $(, $arg_n:expr $(=> $arg_n_post:expr)? )* $(,)?
        )?
    ) => {
        choice((
            $(keyword($name),)*
        ))
        $(
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
        {
            $(({ $post }))?
            (hsp().ignore_then({ $pre }))
        }
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
pub trait E<'a>: P<'a, ()> {}
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

pub fn ident<'a>() -> impl P<'a, Ident> {
    chumsky::text::ident().map(Ident::new)
}

pub fn nat<'a>() -> impl P<'a, Natural> {
    int(10)
        .from_str()
        // expecting the int parser to only accept valid ints
        .unwrapped()
}

/// Allows exactly 2 fractional digits,
/// returns them just left-shifted by 2 digits though
/// (as if the dot was not there).
pub fn decimal<'a>() -> impl P<'a, Natural> {
    let two_digit_num = digits(10)
        .exactly(2)
        .collect()
        .map(|src: String| src.parse::<Natural>())
        // 2 base10 digits are always parsable as BigUint
        .unwrapped();
    nat()
        .then(just('.').ignore_then(two_digit_num))
        // left-shift
        .map(|(whole, frac)| whole * 100u8 + frac)
}

// literals

pub fn gtin<'a>() -> impl P<'a, Gtin> {
    digits(10)
        .at_least(8)
        .at_most(14)
        .collect::<String>()
        .from_str()
        // GTINs can be at most 14 digits long, both max 14 and digits are fulfilled above
        .unwrapped()
}

pub fn ratio<'a>() -> impl P<'a, Ratio> {
    group((
        nat().then_ignore(osp().then(just(":"))),
        osp().ignore_then(nat()),
    ))
    .map(|(left, right)| Ratio { left, right })
}

pub fn cents<'a>() -> impl P<'a, Money> {
    let suffix = osp()
        .then(choice((just("cents"), just("ct"), just("¢"))))
        .or_not();
    nat().then_ignore(suffix).map(Money)
}

pub fn euros<'a>() -> impl P<'a, Money> {
    let value = choice((decimal(), nat().map(|value| value * 100u8)));
    let suffix = osp().then(choice((just("EUR"), just("eur"), just("€"))));

    value.then_ignore(suffix).map(Money)
}

pub fn money<'a>() -> impl P<'a, Money> {
    choice((euros(), cents()))
}

// parameters

pub fn from<'a>() -> impl P<'a, Ident> {
    cmd!("from" : ident()).map(untup)
}

pub fn to<'a>() -> impl P<'a, Ident> {
    cmd!("to" : ident()).map(untup)
}

pub fn dir<'a>() -> impl P<'a, Dir> {
    group((from(), hsp(), to())).map(|(from, _, to)| Dir {
        source: from,
        target: to,
    })
}

pub fn product<'a>() -> impl P<'a, Product> {
    choice((ident().map(Product::Name), gtin().map(Product::Id)))
}

pub fn price<'a>() -> impl P<'a, Money> {
    cmd!("price" : money()).map(untup)
}

pub fn split<'a>() -> impl P<'a, Ratio> {
    cmd!("split" : ratio()).map(untup)
}

// actors

pub fn entity<'a>() -> impl P<'a, Entity> {
    cmd!("entity" : ident()).map(|(name,)| Entity { name })
}

pub fn object<'a>() -> impl P<'a, Object> {
    cmd!(
        "object" :
        ident(),
        cmd!("parent" : ident()).map(untup) => Parser::or_not,
    )
    .map(|(name, parent)| Object { name, parent })
}

pub fn concept<'a>() -> impl P<'a, Concept> {
    cmd!(
        "concept" :
        ident(),
        price() => Parser::or_not,
        cmd!("gtin" : gtin()).map(untup) => Parser::or_not,
    )
    .map(|(name, default_price, gtin)| Concept {
        name,
        default_price,
        gtin,
    })
}

pub fn actor<'a>() -> impl P<'a, Actor> {
    choice((
        entity().map(Actor::Entity),
        object().map(Actor::Object),
        concept().map(Actor::Concept),
    ))
}

// commands

pub fn create<'a>() -> impl P<'a, Create> {
    cmd!("create" : actor()).map(|(who,)| Create { who })
}

pub fn pay<'a>() -> impl P<'a, Pay> {
    cmd!("pay" : money(), dir()).map(|(amount, who)| Pay { amount, who })
}

pub fn deliver<'a>() -> impl P<'a, Deliver> {
    cmd!(
        "deliver" :
        product(),
        price() => Parser::or_not,
        dir(),
        split() => Parser::or_not,
    )
    .map(|(what, price, who, split)| Deliver {
        what,
        price,
        who,
        split,
    })
}

pub fn balance<'a>() -> impl P<'a, Balance> {
    cmd!("balance" | "bal" : dir()).map(|(between,)| Balance { between })
}

pub fn transfer<'a>() -> impl P<'a, Transfer> {
    choice((pay().map(Transfer::Pay), deliver().map(Transfer::Deliver)))
}

pub fn analyze<'a>() -> impl P<'a, Analyze> {
    choice((balance().map(Analyze::Balance),))
}

pub fn statement<'a>() -> impl P<'a, Stmt> {
    choice((
        create().map(Stmt::Create),
        transfer().map(Stmt::Transfer),
        analyze().map(Stmt::Analyze),
    ))
}

// toplevel

/// Upon a `#`, ignore everything until end of line or end of input.
pub fn comment<'a>() -> impl E<'a> {
    just('#')
        // What can appear in a comment?
        .then(any().and_is(newline().not()).repeated())
        // How can comments be ended?
        .then(choice((newline(), end())))
        // Not modeled in the AST.
        .ignored()
}

pub fn delim<'a>() -> impl E<'a> {
    choice((comment(), newline(), just(';').ignored()))
        .padded_by(osp())
        .repeated()
        .at_least(1)
}

pub fn script<'a>() -> impl P<'a, Script> {
    statement()
        .separated_by(delim())
        .allow_leading()
        .allow_trailing()
        .collect()
        .then_ignore(end())
        .map(Script)
}
