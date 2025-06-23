use crate::{
    aux::{Owned, Stack},
    ext::{Gtin, Money, Natural},
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

/// a name in specific is a value, whereas an ident is just somewhere something resembling an
/// identifier
#[derive(Stack!)]
pub struct Name<'tok>(pub Ident<'tok>);

#[derive(Stack!)]
pub struct Ident<'tok>(pub &'tok str);
