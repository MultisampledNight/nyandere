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
                let concept = model::Concept {
                    name: concept.name,
                    default_price: concept.default_price,
                    gtin: concept.gtin,
                };

                // if it has a GTIN, we also need to remember it separately
                // so it is still stored when it is shadowed by name
                if let Some(gtin) = concept.gtin {
                    self.state
                        .concepts_gtin
                        .insert(gtin.clone(), concept.clone());
                }

                self.state.concepts.insert(concept.name.clone(), concept);
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

#[cfg(test)]
mod tests {
    use crate::{Set, eval};

    #[test]
    fn inherit() {
        let state = eval(
            "
            create entity A
            create entity B

            create concept E price 13.37€ gtin 10000000
            create object O parent E
            create object T parent E

            create entity C
            ",
        )
        .unwrap();

        // all entities there?
        assert_eq!(
            state.entities.keys().map(AsRef::as_ref).collect::<Set<_>>(),
            ["A", "B", "C"].into_iter().collect(),
        );

        assert_eq!(
            state
                .objects
                .into_values()
                .map(|o| (o.name.unwrap(), o.parent.unwrap().name))
                .collect::<Set<_>>(),
            Set::from([("O".into(), "E".into()), ("T".into(), "E".into())])
        );
    }
}
