//! Process and understand.
//!
//! What is semantically valid?

pub mod cmd;
pub mod error;
pub mod model;
pub mod repr;

use std::ops::Deref;

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

    pub fn to_state(self) -> State {
        self.state
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
    /// by [representing][Runtime::repr]
    /// every [statement][crate::syntax::ast::Stmt]
    /// as the corresponding [command][cmd::Command]
    /// and running it.
    ///
    /// # Error
    ///
    /// Note that in the case of an error,
    /// the runtime is not rolled back
    /// and still holds the state built _until_ the
    /// invalid instruction.
    pub fn run(&mut self, script: Script) -> Result<(), error::Repr> {
        for stmt in script.0 {
            let cmd = self.repr(stmt)?;
            self.fulfil(cmd);
        }

        Ok(())
    }
}

impl Deref for Runtime {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
