//! Parts flying around that already have a concept of "existing".
//!
//! # On field privacy
//!
//! The fields are all private.
//! Why?
//! The answer is type safety.
//! Actors can only be created by issuing a [`super::cmd::Create`] command
//! to the runtime.
//! This implies that the runtime doesn't need to take care of
//! returning errors about non-existent
//! [`Entity`]ies, [`Concept`]s or [`Object`]s:
//! if there is one, it has to exist and hence be created at some point.

use crate::{
    aux::Owned,
    ext::{Gtin, Money},
};

use super::Name;

/// Someone who holds money and deliver things.
#[derive(Owned!)]
pub struct Entity {
    pub(super) name: Name,
}

/// Designed idea of [`Object`]s.
#[derive(Owned!)]
pub struct Concept {
    pub(super) name: Name,
    pub(super) default_price: Option<Money>,
    pub(super) gtin: Option<Gtin>,
}

impl Concept {
    /// Return a real instanced [`Object`] of this concept.
    pub fn realize(&self) -> Object {
        Object {
            name: None,
            parent: Some(self.clone()),
        }
    }

    /// Return a real instanced [`Object`] of this concept
    /// and give it an accessible name.
    pub fn realize_named(&self, name: Name) -> Object {
        Object {
            name: Some(name),
            parent: Some(self.clone()),
        }
    }
}

/// Physically holdable something.
#[derive(Owned!)]
pub struct Object {
    pub(super) name: Option<Name>,
    pub(super) parent: Option<Concept>,
}

impl Object {
    /// Create a standalone object that has *no* parent [`Concept`].
    pub fn new(name: Name) -> Self {
        Self {
            name: Some(name),
            parent: None,
        }
    }
}

/// Don't care about hypotheticality, the user just wants one *thing*? Use this.
#[derive(Owned!)]
pub enum Product {
    /// Instantiate this concept into an anonymous object.
    Concept(Concept),
    /// Take this object directly.
    Object(Object),
}

impl Product {
    /// Instantiate or directly return the desired [`Object`].
    pub fn realize(&self) -> Object {
        match self {
            Self::Object(object) => object.clone(),
            Self::Concept(concept) => concept.realize(),
        }
    }
}
