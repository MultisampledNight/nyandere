//! What is **semantically** valid?
//!
//! Construct the world and wrap it into a nice usable interface.
//! [`actor`] holds live objects,
//! which can be interacted, created and analyzed
//! via [`runtime`]
//! using types from [`cmd`].

use crate::aux::Owned;

pub mod actor;
pub mod cmd;
pub mod conv;
pub mod runtime;

/// Text-based readable name.
#[derive(Owned!)]
pub struct Name(pub String);

#[derive(Owned!, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    UnknownActor(#[from] UnknownActorError),
}

#[derive(Owned!, thiserror::Error)]
pub enum UnknownActorError {
    #[error("entity {0}")]
    Entity(Name),
    #[error("object {0}")]
    Object(Name),
    #[error("concept {0}")]
    Concept(Name),
}
