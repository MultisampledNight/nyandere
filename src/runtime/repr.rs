use crate::{
    aux::Owned,
    runtime::cmd,
    syntax::ast::{self, Stmt},
};

use super::{
    Runtime,
    cmd::{Command, Name},
};

impl Runtime {
    /// Convert a textually parsed AST (or part of one)
    /// into a semantically valid and meaningful command (or part of one).
    ///
    /// # Errors
    ///
    /// Returns an error when the statement is semantically invalid,
    /// see [`Error`] for details.
    pub fn repr<T, U>(&self, source: T) -> Result<U, Error>
    where
        U: Repr<T>,
    {
        U::repr(source, self)
    }
}

/// [`TryFrom`] but with runtime context. See [`Repr::repr`].
pub trait Repr<T>: Sized {
    /// Convert `T` into [`Self`] in the context of a runtime.
    ///
    /// This is a kind of parser: It narrows `source`s into
    /// valid [`Self`]es and
    /// and invalid values into [`Error`]s.
    fn repr(source: T, runtime: &Runtime) -> Result<Self, Error>;
}

// conversion is trivial if From is already implemented
impl<T, U> Repr<T> for U
where
    T: Into<U>,
{
    fn repr(source: T, _: &Runtime) -> Result<Self, Error> {
        Ok(source.into())
    }
}

#[derive(Owned!, thiserror::Error)]
pub enum Error {
    #[error("unknown actor")]
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

impl Repr<ast::Stmt> for Command {
    fn repr(source: ast::Stmt, runtime: &Runtime) -> Result<Self, Error> {
        use cmd::Command as Cmd;

        let cmd = match source {
            Stmt::Create(cmd) => Cmd::Create(match cmd.who {
                ast::Actor::Entity(entity) => cmd::Create::Entity(entity.into()),
                ast::Actor::Concept(concept) => cmd::Create::Concept(concept.into()),
                ast::Actor::Object(object) => cmd::Create::Object(runtime.repr(object)?),
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

impl Repr<ast::Object> for cmd::Object {
    fn repr(source: ast::Object, runtime: &Runtime) -> Result<Self, Error> {
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
