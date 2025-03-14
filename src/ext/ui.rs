use std::fmt;

use num_bigint::Sign;

use crate::runtime::model::Entity;

use super::{Balance, Debit, Money};

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let whole = &self.0 / 100u8;
        let frac = &self.0 % 100u8;

        write!(f, "{whole}.{frac:02}")
    }
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = match self.0.sign() {
            Sign::Minus => "-",
            _ => "",
        };
        let mag = Money(self.0.magnitude().clone());

        write!(f, "{sign}{mag}")
    }
}

impl fmt::Display for Debit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} owes {} to {}",
            self.between.source(),
            self.amount,
            self.between.target()
        )
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "entity {}", self.name())
    }
}
