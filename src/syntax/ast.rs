use crate::{
    Map,
    ext::{Gtin, Integer},
};

pub struct Script<'tok>(pub Vec<Stmt<'tok>>);

pub struct Stmt<'tok> {
    cmd: Command,
    args: Args<'tok>,
}

pub enum Command {
    Create,
    Pay,
    Deliver,
    Balance,
}

pub struct Args<'tok> {
    pub pos: Vec<Value<'tok>>,
    pub named: Map<Ident<'tok>, Value<'tok>>,
}

pub enum Value<'tok> {
    Money(Money),
    Ratio(Ratio),
    Gtin(Gtin),
    Name(Name<'tok>),
}

pub struct Money(pub Integer);

pub struct Ratio {
    pub from: Integer,
    pub to: Integer,
}

pub struct Name<'tok>(pub Ident<'tok>);

pub type Ident<'tok> = &'tok str;
