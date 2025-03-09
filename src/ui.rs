use std::fmt;

use crate::ext::Money;

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let whole = &self.0 / 100u8;
        let frac = &self.0 % 100u8;

        write!(f, "{whole}.{frac:02}")
    }
}
