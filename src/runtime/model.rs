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

use std::{array::IntoIter, mem};

use crate::{
    Map,
    aux::{NotOrd, Owned},
    ext::{Balance, Gtin, Money},
};

use super::{
    cmd::{Name, NameRef},
    error::{self, UnknownActor, UnknownConcept, UnknownEntity, UnknownObject},
};

// TODO: generate this automatically
#[derive(NotOrd!, Default)]
pub struct State {
    // not much use -- yet, that is
    pub entities: Map<Name, Entity>,
    pub concepts: Map<Name, Concept>,
    pub objects: Map<Name, Object>,

    pub balances: Map<Pair, Balance>,
}

impl State {
    /// Looks up an already created [`Entity`] by name.
    pub fn get_entity(&self, name: NameRef) -> Result<&Entity, UnknownEntity> {
        self.entities
            .get(name)
            .ok_or_else(|| UnknownEntity(name.to_owned()))
    }

    /// Looks up an already created [`Concept`] by name.
    pub fn get_concept(&self, name: NameRef) -> Result<&Concept, UnknownConcept> {
        self.concepts
            .get(name)
            .ok_or_else(|| UnknownConcept(name.to_owned()))
    }

    /// Looks up an already created [`Object`] by name.
    pub fn get_object(&self, name: NameRef) -> Result<&Object, UnknownObject> {
        self.objects
            .get(name)
            .ok_or_else(|| UnknownObject(name.to_owned()))
    }

    pub fn get_dir(&self, source: NameRef, target: NameRef) -> Result<Dir, error::Repr> {
        let lookup = |side| self.get_entity(side).map_err(UnknownActor::Entity).cloned();

        let dir = Dir::new(lookup(source)?, lookup(target)?)?;
        Ok(dir)
    }

    /// Returns how much the [`Dir::source`] owes [`Dir::target`].
    ///
    /// If the balance is _negative_, that means the balance is _in reverse_,
    /// how much [`Dir::target`] owes [`Dir::source`] in absolute value!
    pub fn balance(&self, dir: Dir) -> Balance {
        let mut bal = self
            .balances
            .get(&dir.clone().into())
            .cloned()
            .unwrap_or(Balance(0.into()));

        bal.take_order(dir);
        bal
    }
}

/// Someone who holds money and deliver things.
#[derive(Owned!)]
pub struct Entity {
    pub(super) name: Name,
}

impl Entity {
    pub fn name(&self) -> NameRef {
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
    pub fn name(&self) -> NameRef {
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
    pub fn name(&self) -> Option<NameRef> {
        self.name.as_ref().map(String::as_ref)
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
    pub(super) source: Entity,
    pub(super) target: Entity,
}

impl Dir {
    /// Tries to construct a directed edge from `source` to `target`.
    ///
    /// # Errors
    ///
    /// Returns a [`SameError`] iff the two entities are the same.
    pub fn new(source: Entity, target: Entity) -> Result<Self, error::Same> {
        if source == target {
            return Err(error::Same(source, target));
        }
        Ok(Self { source, target })
    }

    pub fn source(&self) -> &Entity {
        &self.source
    }

    pub fn target(&self) -> &Entity {
        &self.target
    }

    /// Returns [`true`]
    /// iff converting to a [`Pair`]
    /// would put `target` as first argument
    /// and `source` as second,
    /// otherwise the other way around
    pub fn would_reorder(&self) -> bool {
        self.source > self.target
    }

    /// Exchanges `source` and `target`.
    pub fn flip(&mut self) {
        mem::swap(&mut self.source, &mut self.target);
    }
}

impl From<Dir> for Pair {
    fn from(Dir { source, target }: Dir) -> Self {
        Self::new(source, target).unwrap()
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
    // invariant: a <= b
    pub(super) a: Entity,
    pub(super) b: Entity,
}

impl Pair {
    /// Tries to construct an **undirected** edge between `a` and `b`.
    /// The order does not matter:
    /// the pair `a`, `b` is equivalent to the pair `b`, `a`.
    ///
    /// # Errors
    ///
    /// Returns a [`SameError`] iff the two entities are the same.
    pub fn new(a: Entity, b: Entity) -> Result<Self, error::Same> {
        // might seem needlessly complicated
        // but i want to parametrize this as much as possible
        let dir = Dir::new(a, b)?;
        let reorder = dir.would_reorder();
        let [a, b] = dir.into();

        let (a, b) = if reorder { (b, a) } else { (a, b) };

        Ok(Self { a, b })
    }

    pub fn a(&self) -> &Entity {
        &self.a
    }

    pub fn b(&self) -> &Entity {
        &self.b
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
