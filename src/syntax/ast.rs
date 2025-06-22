use crate::{
    Map,
    aux::{Owned, Stack},
    ext::{Gtin, Integer},
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
pub struct Args<'tok> {
    pub pos: Vec<Value<'tok>>,
    pub named: Map<Ident<'tok>, Value<'tok>>,
}

#[derive(Owned!)]
pub enum Value<'tok> {
    Money(Money),
    Ratio(Ratio),
    Gtin(Gtin),
    Name(Name<'tok>),
}

#[derive(Owned!)]
pub struct Money(pub Integer);

#[derive(Owned!)]
pub struct Ratio {
    pub from: Integer,
    pub to: Integer,
}

#[derive(Stack!)]
pub struct Name<'tok>(pub Ident<'tok>);

pub type Ident<'tok> = &'tok str;
