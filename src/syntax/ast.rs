use crate::{
    Map,
    aux::{Owned, Stack},
    ext::{Gtin, Integer, Money},
};

#[derive(Owned!)]
pub struct Script<'tok>(pub Vec<Stmt<'tok>>);

#[derive(Owned!)]
pub struct Stmt<'tok> {
    pub cmd: Command,
    pub args: Args<'tok>,
}

#[derive(Owned!)]
pub enum Command {
    Create,
    Pay,
    Deliver,
    Balance,
}

#[derive(Owned!)]
pub struct Args<'tok>(pub Vec<Arg<'tok>>);

#[derive(Owned!)]
pub enum Arg<'tok> {
    Named {
        key: Ident<'tok>,
        value: Value<'tok>,
    },
    Pos(Value<'tok>),
}

#[derive(Owned!)]
pub enum Value<'tok> {
    Money(Money),
    Split(Split),
    Gtin(Gtin),
    Name(Name<'tok>),
}

#[derive(Owned!)]
pub struct Split {
    pub from: Natural,
    pub to: Natural,
}

#[derive(Stack!)]
pub struct Name<'tok>(pub Ident<'tok>);

pub type Ident<'tok> = &'tok str;
