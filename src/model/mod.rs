//! What is **semantically** valid?
//!
//! Construct the world and wrap it into a nice usable interface.
//!
//! [`actor`] holds live objects,
//! which can be interacted, created and analyzed
//! via [`runtime`]
//! using types from [`cmd`].

pub mod actor;
pub mod cmd;
pub mod runtime;

mod encode;
