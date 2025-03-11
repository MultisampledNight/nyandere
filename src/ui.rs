use std::fmt;

use crate::{ext::Money, model::Name};

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let whole = &self.0 / 100u8;
        let frac = &self.0 % 100u8;

        write!(f, "{whole}.{frac:02}")
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}`", self.0)
    }
}
