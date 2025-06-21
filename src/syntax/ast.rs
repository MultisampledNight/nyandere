use crate::{
    Map,
    ext::{Gtin, Integer},
};

pub struct Script(pub Vec<Stmt>);

pub struct Stmt {
    cmd: Command,
    args: Args,
}

pub enum Command {
    Create,
    Pay,
    Deliver,
    Balance,
}

pub struct Args<'src> {
    pub pos: Vec<Value<'src>>,
    pub named: Map<Ident<'src>, Value<'src>>,
}

pub enum Value<'a> {
    Money(Money),
    Ratio(Ratio),
    Gtin(Gtin),
    Name(Name<'a>),
}

pub struct Money(pub Integer);

pub struct Ratio {
    pub from: Integer,
    pub to: Integer,
}

pub struct Name<'src>(pub Ident<'src>);

pub type Ident<'src> = &'src str;
