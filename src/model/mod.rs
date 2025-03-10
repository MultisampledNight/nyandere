//! What is **semantically** valid?
//!
//! Construct the world and wrap it into a nice usable interface.
//!
//! # Why do [`actor`] and [`cmd`] contain almost identical types?!
//!
//! [`actor`] purely handles **already existing** things,
//! while [`cmd`] contains everything that is needed for *interaction*.

use crate::aux::Owned;

pub mod actor;
pub mod cmd;

/// Text-based readable name.
#[derive(Owned!)]
pub struct Name(pub String);
