//! Construct the world and wrap it into a nice usable interface.

use crate::{aux::{Common, Owned}, ext::Money};

/// Do something that makes, modifies or reads.
#[derive(Common!)]
pub enum Command {
    Create,
    Pay,
    Deliver,
    Purchase,
    Stats,
    Balance,
}

/// Directed edge between 2 [`Entity`]ies.
#[derive(Owned!)]
pub struct Dir {
    pub from: Entity,
    pub to: Entity,
}

/// Someone who holds money and deliver things.
#[derive(Owned!)]
pub struct Entity {
    name: Name,
}

/// Designed idea of [`Object`]s.
#[derive(Owned!)]
pub struct Concept {
    pub name: Name,
    pub default_price: Option<Money>,
}

/// Physically holdable something.
#[derive(Owned!)]
pub struct Object {
    pub name: Option<Name>,
    pub parent: Option<Concept>,
}

/// Don't care about hypotheticality, the user just wants one *thing*? Use this.
pub enum Product {
    /// Instantiate this concept into an anonymous object.
    Concept(Concept),
    /// Take this object directly.
    Object(Object),
}

/// Text-based readable name.
#[derive(Owned!)]
pub struct Name(pub String);
