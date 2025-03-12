//! Structurally interact with, modify and do things.

pub mod create;

pub use create::Create;

use crate::{
    aux::Owned,
    ext::{Gtin, Money},
};

use super::{Runtime, model};

// TODO: do this via dynamic dispatch so the cases don't have to be matched manually?

impl Runtime {
    /// Performs one single command.
    ///
    /// This can never fail, any [`Command`]
    /// ***that is constructed from this instance***
    /// is valid to run at any point after construction!
    pub fn fulfil(&mut self, cmd: Command) {
        use Command as C;
        match cmd {
            C::Create(create) => self.create(create),
            _ => todo!(),
        }
    }
}

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
    pub parent: Option<model::Concept>,
}

/// Directed edge between 2 [`Entity`]ies.
#[derive(Owned!)]
pub struct Dir {
    pub from: Entity,
    pub to: Entity,
}

/// Text-based readable name.
#[derive(Owned!)]
pub struct Name(pub String);

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
