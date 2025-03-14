//! All kinds of runtime errors.

use thiserror::Error;

use crate::aux::Owned;

use super::{cmd::Name, model::Entity};

#[derive(Owned!, thiserror::Error)]
pub enum Repr {
    #[error("unknown actor")]
    UnknownActor(#[from] UnknownActor),
    // reasoning: there is no reason for a noop in money processing. likely a typo
    #[error("from and to are the same, would be a noop")]
    Same(#[from] Same),
}

#[derive(Owned!, Error)]
#[error("{0} and {1} are the same, but mustn't be")]
pub struct Same(pub Entity, pub Entity);

#[derive(Owned!, Error)]
#[error("unknown actor -- maybe a typo? if you're sure it's not one, create it")]
pub enum UnknownActor {
    Entity(#[from] UnknownEntity),
    Concept(#[from] UnknownConcept),
    Object(#[from] UnknownObject),
}

#[derive(Owned!, thiserror::Error)]
#[error("unknown entity {0}")]
pub struct UnknownEntity(pub Name);

#[derive(Owned!, thiserror::Error)]
#[error("unknown concept {0}")]
pub struct UnknownConcept(pub Name);

#[derive(Owned!, thiserror::Error)]
#[error("unknown object {0}")]
pub struct UnknownObject(pub Name);
