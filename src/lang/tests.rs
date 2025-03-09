use std::fmt;

use chumsky::Parser;

use super::{ast::*, parse::*};

fn assert<'a, T: fmt::Debug + PartialEq>(parser: impl P<'a, T>, src: &'a str, intended: T) {
    let output = parser.parse(src).unwrap();
    assert_eq!(output, intended);
}

#[test]
fn snapshot() {
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
