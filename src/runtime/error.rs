//! All kinds of runtime errors.

use thiserror::Error;

use crate::{aux::Owned, ext::Gtin};

use super::{
    cmd::Name,
    model::{Entity, Product},
};

#[derive(Owned!, thiserror::Error)]
#[error("could not semantically understand input")]
pub enum Repr {
    UnknownActor(#[from] UnknownActor),
    Same(#[from] Same),
    DeliveryPaymentUnclear(#[from] PriceUnspecified),
}

#[derive(Owned!, Error)]
#[error("unknown actor -- maybe a typo? if you're sure it's not one, create it")]
pub enum UnknownActor {
    Entity(#[from] UnknownEntity),
    Concept(#[from] UnknownConcept),
    ConceptGtin(#[from] UnknownConceptGtin),
    Object(#[from] UnknownObject),
    ProductName(#[from] UnknownProductName),
}

#[derive(Owned!, thiserror::Error)]
#[error("unknown entity {0}")]
pub struct UnknownEntity(pub Name);

#[derive(Owned!, thiserror::Error)]
#[error("unknown concept {0}")]
pub struct UnknownConcept(pub Name);

#[derive(Owned!, thiserror::Error)]
#[error("unknown concept {0}")]
pub struct UnknownConceptGtin(pub Gtin);

#[derive(Owned!, thiserror::Error)]
#[error("unknown object {0}")]
pub struct UnknownObject(pub Name);

#[derive(Owned!, thiserror::Error)]
#[error("unknown product {0} (is neither an object name nor a concept name)")]
pub struct UnknownProductName(pub Name);

/// There is no reason for a noop in money processing. Likely a typo.
#[derive(Owned!, Error)]
#[error("{0} and {1} are the same, but mustn't be")]
pub struct Same(pub Entity, pub Entity);

/// The price of that product is not specified and
/// could not be inferred.
///
/// If that is intentional as it's a gift, specify `0` as price.
/// Otherwise, see if you might need to update a concept definition.
#[derive(Owned!, thiserror::Error)]
#[error("cannot deliver {product} without knowing the money expected in return at some point -- specify 0 if it's a gift")]
pub struct PriceUnspecified {
    pub product: Product,
}
