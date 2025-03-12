//! Process and understand.
//!
//! What is semantically valid?

pub mod cmd;
pub mod encode;
pub mod model;

use cmd::Command;
use encode::Encoded;
pub use model::State;

use crate::{Script, aux::NotOrd};

#[derive(NotOrd!, Default)]
pub struct Runtime {
    state: State,
}

impl Runtime {
    pub fn state(&self) -> &State {
        &self.state
    }
}

impl Runtime {
    /// Initializes an empty runtime.
    ///
    /// Run [`cmd::Create`] to start filling it afterwards.
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluate a whole parsed [`Script`]
    /// by [encoding][Runtime::encode]
    /// every [statement][crate::syntax::ast::Stmt]
    /// into the corresponding [command][Command]
    /// and running it.
    ///
    /// # Error
    ///
    /// Note that in the case of an error,
    /// the runtime is not rolled back
    /// and still holds the state built _until_ the
    /// invalid instruction.
    pub fn run(&mut self, script: Script) -> Result<(), encode::Error> {
        for stmt in script.0 {
            let cmd = self.encode(stmt)?;
            self.apply(cmd);
        }

        Ok(())
    }

    /// Convert a textually parsed AST (or part of one)
    /// into a semantically valid and meaningful command (or part of one).
    ///
    /// # Errors
    ///
    /// Returns an error when the statement is semantically invalid,
    /// see [`Error`] for details.
    pub fn encode<T, U>(&self, source: T) -> Result<U, encode::Error>
    where
        U: Encoded<T>,
    {
        U::encoded(source, self)
    }

    /// Performs one single command.
    ///
    /// This can never fail, any [`Command`]
    /// ***that is constructed from this instance***
    /// is valid to run at any point after construction!
    pub fn apply(&mut self, cmd: Command) {
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
