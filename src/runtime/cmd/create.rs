use crate::{Runtime, aux::Owned, runtime::model};

use super::{Concept, Entity, Object};

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
///
/// # Caveats
///
/// Creating another actor with the same name
/// replaces the previous actor
/// while also replacing all references
/// (but not previous payments).
#[derive(Owned!)]
pub enum Create {
    Entity(Entity),
    Concept(Concept),
    Object(Object),
}

impl Runtime {
    pub fn create(&mut self, cmd: Create) {
        use Create as C;
        match cmd {
            C::Entity(entity) => {
                self.state
                    .entities
                    .insert(entity.name.clone(), model::Entity { name: entity.name });
            }
            C::Concept(concept) => {
                self.state.concepts.insert(
                    concept.name.clone(),
                    model::Concept {
                        name: concept.name,
                        default_price: concept.default_price,
                        gtin: concept.gtin,
                    },
                );
            }
            C::Object(object) => {
                self.state.objects.insert(
                    object.name.clone(),
                    model::Object {
                        name: Some(object.name),
                        parent: object.parent,
                    },
                );
            }
        }
    }
}
