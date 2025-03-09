use std::fmt;

use chumsky::Parser;

use super::{ast::*, parse::*};

fn assert<'a, T>(parser: impl P<'a, T>, src: &'a str, intended: T)
where
    T: fmt::Debug + PartialEq,
{
    let output = parser.parse(src).unwrap();
    assert_eq!(output, intended);
}

#[test]
fn basic() {
    assert(
        comment(),
        "# this is one single comment — even with ✨ special ✨ emojis",
        (),
    );
    assert(
        pay(),
        "pay 30ct from A to B",
        Pay {
            amount: Money(30u8.into()),
            who: Dir {
                from: Ident::new("A"),
                to: Ident::new("B"),
            },
        },
    );
}

#[test]
fn stonks() {
    // all of these should be the same!
    for src in ["1337", "1337¢", "1337 ct", "13.37€"] {
        assert(money(), src, Money(1337u16.into()));
    }

    // what about a non-fractional euro?
    assert(money(), "1 EUR", Money(100u16.into()));

    // what about absurdly large numbers?
    let src = u128::MAX.to_string();
    assert(money(), &src, Money(u128::MAX.into()));
}
