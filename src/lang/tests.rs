use std::fmt;

use chumsky::{Parser, prelude::*};

use crate::ext::{Gtin, Money};

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
    let a = || Ident::new("A");
    let b = || Ident::new("B");
    let gtin = 12345678901234;

    assert(
        comment().ignore_then(just("abc")),
        "# this is a comment with ✨ special ✨ emojis\nabc",
        "abc",
    );

    assert(
        pay(),
        "pay 30ct from A to B",
        Pay {
            amount: Money(30u8.into()),
            who: Dir { from: a(), to: b() },
        },
    );

    assert(
        deliver(),
        &format!("deliver {gtin} price 1€ from A to B"),
        Deliver {
            what: Product::Id(Gtin::new(gtin).unwrap()),
            who: Dir { from: a(), to: b() },
            price: Some(Money(100u8.into())),
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
