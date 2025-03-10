//! Things to do that somehow interact with actors.

use super::{actor, Name};

use crate::{
    aux::Owned,
    ext::{Gtin, Money},
};

/// Do something that makes, modifies or reads.
#[derive(Owned!)]
pub enum Command {
    Create(Create),
    Pay,
    Deliver,
    Purchase,
    Stats,
    Balance,
}

/// Introduce a new actor.
///
/// This is required for any actor
/// before it can be used.
/// While this does create some noise,
/// it has 2 advantages:
///
/// 1. It reduces the likelihood of typos causing damage.
///     Money handling applications
///     are definitely not ones
///     where one wants to have some
///     payment not included
///     because the finger slipped one key.
/// 2. It allows post-reference.
///     For example,
///     [`Concept`] can store a default price
///     to use when none is specified and
///     a GTIN it is referred to by.
///     This allows using the default price
///     when just scanning a GTIN!
#[derive(Owned!)]
pub enum Create {
    Entity(Entity),
    Concept(Concept),
    Object(Object),
}

#[derive(Owned!)]
pub struct Entity {
    pub name: Name,
}

#[derive(Owned!)]
pub struct Concept {
    pub name: Name,
    pub default_price: Option<Money>,
    pub gtin: Option<Gtin>,
}

#[derive(Owned!)]
pub struct Object {
    pub name: Name,
    pub parent: Option<actor::Concept>,
}

/// Directed edge between 2 [`Entity`]ies.
#[derive(Owned!)]
pub struct Dir {
    pub from: Entity,
    pub to: Entity,
}
