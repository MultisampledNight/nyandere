use crate::{aux::Owned, ext::Money};

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
    name: Option<Name>,
    parent: Option<Concept>,
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

/// Text-based readable name.
#[derive(Owned!)]
pub struct Name(pub String);
