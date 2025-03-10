//! An [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree).
//!
//! First layer of abstraction over the source text.

use crate::{
    aux::Owned,
    ext::{Gtin, Money},
};

#[derive(Owned!)]
pub struct Script(pub Vec<Stmt>);

/// Something that can be done.
#[derive(Owned!)]
pub enum Stmt {
    Create(Create),
    Transfer(Transfer),
    Analyze(Analyze),
}

/// Commands that introduce new state.
#[derive(Owned!)]
pub struct Create {
    pub who: Actor,
}

/// Actions that do something and modify state.
#[derive(Owned!)]
pub enum Transfer {
    Pay(Pay),
    Deliver(Deliver),
    Purchase(Purchase),
}

/// Read-only commands.
#[derive(Owned!)]
pub enum Analyze {
    Stats(Stats),
    Balance(Balance),
}

/// Money transfer.
#[derive(Owned!)]
pub struct Pay {
    pub amount: Money,
    pub who: Dir,
}

/// Physical transfer implying a money transfer.
#[derive(Owned!)]
pub struct Deliver {
    pub what: Product,
    pub who: Dir,
    pub price: Option<Money>,
}

/// Delivery and payment back in one go,
/// not influencing balance hence.
#[derive(Owned!)]
pub struct Purchase {
    pub what: Product,
    pub who: Dir,
    pub price: Option<Money>,
}

#[derive(Owned!)]
pub struct Stats;

#[derive(Owned!)]
pub struct Balance {
    pub between: Dir,
}

#[derive(Owned!)]
pub enum Actor {
    Entity(Entity),
    Object(Object),
    Concept(Concept),
}

/// Holds money and resources.
#[derive(Owned!)]
pub struct Entity {
    pub name: Ident,
}

/// Can be delivered and passed around.
#[derive(Owned!)]
pub struct Object {
    pub name: Ident,
    pub parent: Option<Ident>,
}

#[derive(Owned!)]
pub struct Concept {
    pub name: Ident,
    pub default_price: Option<Money>,
    pub gtin: Option<Gtin>,
}

#[derive(Owned!)]
pub enum Product {
    Name(Ident),
    Id(Gtin),
}

/// Directional specification.
/// Source and recipient.
#[derive(Owned!)]
pub struct Dir {
    /// Who gives something away.
    pub from: Ident,

    /// Who receives it.
    pub to: Ident,
}

#[derive(Owned!)]
pub struct Ident(String);

impl Ident {
    /// Use for parsing only.
    pub(super) fn new(ident: impl AsRef<str>) -> Self {
        Self(ident.as_ref().to_string())
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}
