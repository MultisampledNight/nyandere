use chumsky::{
    extra::Err,
    prelude::*,
    span::Span,
    text::{digits, inline_whitespace},
};

use super::ast::{Money, Script};

/// Alias for [`chumsky::Parser`] so we don't need to write out the input type all the time.
pub trait P<'a, Node>: chumsky::Parser<'a, &'a str, Node, Err<Rich<'a, char, SimpleSpan>>> {}
impl<'a, Node, T> P<'a, Node> for T where
    T: chumsky::Parser<'a, &'a str, Node, Err<Rich<'a, char, SimpleSpan>>>
{
}

pub fn script<'a>() -> impl P<'a, Script> {
    todo()
}

pub fn euros<'a>() -> impl P<'a, Money> {
    todo()
}

/// Positive integer.
// TODO: make this return a bigint instead
pub fn natural<'a>() -> impl P<'a, u64> {
    digits(10).map(|ch: char| ch)
}

/// Optional whitespace.
pub fn osp<'a>() -> impl P<'a, ()> {
    inline_whitespace()
}

/// Required whitespace.
pub fn hsp<'a>() -> impl P<'a, ()> {
    inline_whitespace().at_least(1)
}
