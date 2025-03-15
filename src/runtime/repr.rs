//! Refine what's semantically meaningful and what not.

use crate::{
    runtime::cmd,
    syntax::ast::{self, Stmt},
};

use super::{
    Runtime,
    cmd::{Balance, Command, Deliver, Pay},
    error,
    model::{Dir, Product},
};

impl Runtime {
    /// Convert a textually parsed AST (or part of one)
    /// into a semantically valid and meaningful command (or part of one).
    ///
    /// # Errors
    ///
    /// Returns an error when the statement is semantically invalid,
    /// see [`error::Repr`] for details.
    pub fn repr<T, U>(&self, source: T) -> Result<U, error::Repr>
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
    /// and invalid values into [`error::Repr`]s.
    fn repr(source: T, runtime: &Runtime) -> Result<Self, error::Repr>;
}

// conversion is trivial if From is already implemented
impl<T, U> Repr<T> for U
where
    T: Into<U>,
{
    fn repr(source: T, _: &Runtime) -> Result<Self, error::Repr> {
        Ok(source.into())
    }
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
    fn repr(source: ast::Stmt, runtime: &Runtime) -> Result<Self, error::Repr> {
        use cmd::Command as Cmd;

        let cmd = match source {
            Stmt::Create(cmd) => Cmd::Create(match cmd.who {
                ast::Actor::Entity(entity) => cmd::Create::Entity(entity.into()),
                ast::Actor::Concept(concept) => cmd::Create::Concept(concept.into()),
                ast::Actor::Object(object) => cmd::Create::Object(runtime.repr(object)?),
            }),
            Stmt::Transfer(cmd) => match cmd {
                ast::Transfer::Pay(cmd) => Cmd::Pay(runtime.repr(cmd)?),
                ast::Transfer::Deliver(cmd) => Cmd::Deliver(runtime.repr(cmd)?),
                _ => todo!(),
            },
            Stmt::Analyze(cmd) => match cmd {
                ast::Analyze::Balance(cmd) => Cmd::Balance(runtime.repr(cmd)?),
                _ => todo!(),
            },
        };

        Ok(cmd)
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
    fn repr(source: ast::Object, runtime: &Runtime) -> Result<Self, error::Repr> {
        Ok(Self {
            name: source.name.into(),
            // not having a parent concept is entirely valid
            // the only errornuous case is a referenced parent that doesn't exist
            parent: source
                .parent
                // maybe be variadic over the return type in Repr?
                // e.g. i want to look up Concepts while inputting Names
                // or should those be methods?
                .map(|parent| runtime.get_concept(parent.as_ref()).cloned())
                .transpose()
                .map_err(error::UnknownActor::Concept)?,
        })
    }
}

impl Repr<ast::Pay> for Pay {
    fn repr(source: ast::Pay, runtime: &Runtime) -> Result<Self, error::Repr> {
        Ok(Self {
            amount: source.amount,
            who: runtime.repr(source.who)?,
        })
    }
}

impl Repr<ast::Deliver> for Deliver {
    fn repr(source: ast::Deliver, runtime: &Runtime) -> Result<Self, error::Repr> {
        // NOTE: do not move this into the chain below.
        // this check should *always* happen on delivery.
        // there's no purpose in delivering something that doesn't
        // exist. we'll keep track of the deliveries in future.
        let product: Product = runtime.repr(source.what)?;

        // goal: find the price of the delivered product
        let price =
            // is it directly the deliver command itself?
            source.price
            // if not, is there a known default price for the product?
            .map_or_else(|| product.default_price().cloned(), Ok)?;
        Ok(Self {
            who: runtime.repr(source.who)?,
            price,
        })
    }
}

impl Repr<ast::Balance> for Balance {
    fn repr(source: ast::Balance, runtime: &Runtime) -> Result<Self, error::Repr> {
        Ok(Self {
            between: runtime.repr(source.between)?,
        })
    }
}

impl Repr<ast::Dir> for Dir {
    fn repr(ast::Dir { from, to }: ast::Dir, runtime: &Runtime) -> Result<Self, error::Repr> {
        runtime.get_dir(from.as_ref(), to.as_ref())
    }
}

impl Repr<ast::Product> for Product {
    fn repr(source: ast::Product, runtime: &Runtime) -> Result<Self, error::Repr> {
        match source {
            ast::Product::Id(gtin) => Ok(Product::Concept(
                runtime
                    .get_concept_by_gtin(&gtin)
                    .map_err(error::UnknownActor::ConceptGtin)?
                    .clone(),
            )),
            ast::Product::Name(name) => {
                // TODO: document somewhere that by name, objects are looked up before concepts are
                let name = name.as_ref();
                let product = match (runtime.get_object(name), runtime.get_concept(name)) {
                    (Ok(object), _) => Product::Object(object.clone()),
                    (_, Ok(concept)) => Product::Concept(concept.clone()),
                    (Err(_), Err(_)) => {
                        // i wish that some day we get some autopathfinding for errors in rust
                        // this is. um. not easy to read nor fun to write
                        return Err(error::UnknownActor::ProductName(error::UnknownProductName(
                            name.to_owned(),
                        )))?;
                    }
                };

                Ok(product)
            }
        }
    }
}
