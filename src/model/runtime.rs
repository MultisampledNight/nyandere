use std::collections::HashMap;

use crate::aux::NotOrd;

use super::{Name, actor, cmd};

#[derive(NotOrd!, Default)]
pub struct Runtime {
    state: State,
}

impl Runtime {
    pub fn state(&self) -> &State {
        &self.state
    }
}

// TODO: generate this automatically
#[derive(NotOrd!, Default)]
pub struct State {
    // not much use -- yet, that is
    pub entities: HashMap<Name, actor::Entity>,
    pub concepts: HashMap<Name, actor::Concept>,
    pub objects: HashMap<Name, actor::Object>,
}

impl Runtime {
    /// Initializes an empty runtime.
    ///
    /// Run [`cmd::Create`] to start filling it afterwards.
    pub fn new() -> Self {
        Self::default()
    }

    /// Runs one command.
    pub fn run(&mut self, cmd: cmd::Command) {
        use cmd::Command as C;
        match cmd {
            C::Create(create) => self.create(create),
            _ => todo!(),
        }
    }

    pub fn create(&mut self, cfg: cmd::Create) {
        use cmd::Create as C;
        match cfg {
            C::Entity(entity) => {
                self.state
                    .entities
                    .insert(entity.name.clone(), actor::Entity { name: entity.name });
            }
            C::Concept(concept) => {
                self.state.concepts.insert(
                    concept.name.clone(),
                    actor::Concept {
                        name: concept.name,
                        default_price: concept.default_price,
                        gtin: concept.gtin,
                    },
                );
            }
            C::Object(object) => {
                self.state.objects.insert(
                    object.name.clone(),
                    actor::Object {
                        name: Some(object.name),
                        parent: object.parent,
                    },
                );
            }
        }
    }
}
