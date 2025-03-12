use crate::{
    aux::Owned,
    syntax::ast::{self, Stmt},
};

use super::{
    cmd::{self, Command, Name},
    runtime::Runtime,
};

impl Runtime {
    /// Convert a textually parsed AST (or part of one)
    /// into a semantically valid and meaningful command (or part of one).
    ///
    /// # Errors
    ///
    /// Returns an error when the statement is semantically invalid,
    /// see [`EncodeError`] for details.
    pub fn encode<T, U>(&self, source: T) -> Result<U, Error>
    where
        U: Encoded<T>,
    {
        U::encoded(source, self)
    }
}

/// [`TryFrom`] but with runtime context. See [`Encode::encode`].
pub trait Encoded<T>: Sized {
    /// Convert `T` into [`Self`] in the context of a runtime.
    ///
    /// This is a kind of parser: It narrows `source`s into
    /// valid [`Self`]es and
    /// and invalid values into [`Error`]s.
    fn encoded(source: T, runtime: &Runtime) -> Result<Self, Error>;
}

impl<T, U> Encoded<T> for U
where
    T: Into<U>,
{
    fn encoded(source: T, _: &Runtime) -> Result<Self, Error> {
        Ok(source.into())
    }
}

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

// this is so noisy, ramblings about that:
// ideally e.g. `cmd::Command` would impl `From<cmd::Create>` and
// `cmd::Create::Entity` impl `From<Entity>`
// generating that automatically would probably just be a few proc macros
// but i did not find any crate to do so
// at some later point i might write one. even a decl macro would suffice
// (generate `From` impls for all tuple struct variants with exactly 1 value)
// but not today

impl Encoded<ast::Stmt> for Command {
    fn encoded(source: ast::Stmt, runtime: &Runtime) -> Result<Self, Error> {
        use cmd::Command as Cmd;

        let cmd = match source {
            Stmt::Create(cmd) => Cmd::Create(match cmd.who {
                ast::Actor::Entity(entity) => cmd::Create::Entity(entity.into()),
                ast::Actor::Concept(concept) => cmd::Create::Concept(concept.into()),
                ast::Actor::Object(object) => cmd::Create::Object(runtime.encode(object)?),
            }),
            _ => todo!(),
        };

        Ok(cmd)
    }
}

impl From<ast::Ident> for Name {
    fn from(ident: ast::Ident) -> Self {
        Self(ident.take())
    }
}

impl From<ast::Entity> for cmd::Entity {
    fn from(ast::Entity { name }: ast::Entity) -> Self {
        Self { name: name.into() }
    }
}

impl From<ast::Concept> for cmd::Concept {
    fn from(
        ast::Concept {
            name,
            default_price,
            gtin,
        }: ast::Concept,
    ) -> Self {
        Self {
            name: name.into(),
            default_price,
            gtin,
        }
    }
}

impl Encoded<ast::Object> for cmd::Object {
    fn encoded(source: ast::Object, runtime: &Runtime) -> Result<Self, Error> {
        Ok(Self {
            name: source.name.into(),
            // not having a parent concept is entirely valid
            // the only errornuous case is a referenced parent that doesn't exist
            parent: source
                .parent
                .map(Into::into)
                .map(|parent| {
                    runtime
                        .state()
                        .concepts
                        .get(&parent)
                        .cloned()
                        .ok_or(Error::UnknownActor(UnknownActorError::Concept(parent)))
                })
                .transpose()?,
        })
    }
}
