//! Live actors, bundled, cuddled and wrapped up into [`State`].
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

use std::{array::IntoIter, cmp::Ordering};

use thiserror::Error;

use crate::{
    Map,
    aux::{NotOrd, Owned},
    ext::{Gtin, Money},
};

use super::cmd::Name;

// TODO: generate this automatically
#[derive(NotOrd!, Default)]
pub struct State {
    // not much use -- yet, that is
    pub entities: Map<Name, Entity>,
    pub concepts: Map<Name, Concept>,
    pub objects: Map<Name, Object>,

    pub balance: Map<Pair, Money>,
}

/// Someone who holds money and deliver things.
#[derive(Owned!)]
pub struct Entity {
    pub(super) name: Name,
}

impl Entity {
    pub fn name(&self) -> &Name {
        &self.name
    }
}

/// Designed idea of [`Object`]s.
#[derive(Owned!)]
pub struct Concept {
    pub(super) name: Name,
    pub(super) default_price: Option<Money>,
    pub(super) gtin: Option<Gtin>,
}

impl Concept {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn default_price(&self) -> Option<&Money> {
        self.default_price.as_ref()
    }

    pub fn gtin(&self) -> Option<Gtin> {
        self.gtin
    }

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
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn parent(&self) -> Option<&Concept> {
        self.parent.as_ref()
    }

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

/// **Directed** edge between 2 different [`Entity`]ies.
#[derive(Owned!)]
pub struct Dir {
    source: Entity,
    target: Entity,
}

impl Dir {
    pub fn source(&self) -> &Entity {
        &self.source
    }

    pub fn target(&self) -> &Entity {
        &self.target
    }

    /// Tries to construct a directed edge from `source` to `target`.
    ///
    /// # Errors
    ///
    /// Returns a [`SameError`] iff the two entities are the same.
    pub fn new(source: Entity, target: Entity) -> Result<Self, SameError> {
        if source == target {
            return Err(SameError(source, target));
        }
        Ok(Self { source, target })
    }
}

impl From<Dir> for [Entity; 2] {
    fn from(Dir { source, target }: Dir) -> Self {
        [source, target]
    }
}

impl IntoIterator for Dir {
    type Item = Entity;
    type IntoIter = IntoIter<Entity, 2>;
    fn into_iter(self) -> Self::IntoIter {
        <[Entity; 2]>::from(self).into_iter()
    }
}

/// **Undirected** edge between 2 different [`Entity`]ies.
#[derive(Owned!)]
pub struct Pair {
    // invariant: a < b
    a: Entity,
    b: Entity,
}

impl Pair {
    pub fn a(&self) -> &Entity {
        &self.a
    }

    pub fn b(&self) -> &Entity {
        &self.b
    }

    /// Tries to construct an **undirected** edge between `a` and `b`.
    /// The order does not matter:
    /// the pair `a`, `b` is equivalent to the pair `b`, `a`.
    ///
    /// # Errors
    ///
    /// Returns a [`SameError`] iff the two entities are the same.
    pub fn new(a: Entity, b: Entity) -> Result<Self, SameError> {
        use Ordering as O;

        let (a, b) = match a.cmp(&b) {
            O::Equal => return Err(SameError(a, b)),
            // guarantee consistent ordering
            // manual sorting, in a way
            O::Less => (a, b),
            O::Greater => (b, a),
        };

        Ok(Self { a, b })
    }
}

impl From<Pair> for [Entity; 2] {
    fn from(Pair { a, b }: Pair) -> Self {
        [a, b]
    }
}

impl IntoIterator for Pair {
    type Item = Entity;
    type IntoIter = IntoIter<Entity, 2>;
    fn into_iter(self) -> Self::IntoIter {
        <[Entity; 2]>::from(self).into_iter()
    }
}

#[derive(Owned!, Error)]
#[error("entities {0} and {1} are the same, but mustn't be")]
pub struct SameError(Entity, Entity);
