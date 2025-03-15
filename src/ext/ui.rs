use std::fmt;

use num_bigint::Sign;

use crate::runtime::model::{Concept, Entity, Object, Product};

use super::{Balance, Debit, Gtin, Money};

impl fmt::Display for Gtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gtin {} ({} digits)", self.get(), self.digits())
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let whole = &self.0 / 100u8;
        let frac = &self.0 % 100u8;

        write!(f, "{whole}.{frac:02} €")
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

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Product::Concept(concept) => write!(f, "{concept}"),
            Product::Object(object) => write!(f, "{object}"),
        }
    }
}

impl fmt::Display for Concept {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "concept {}", self.name())
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "object")?;
        if let Some(name) = self.name() {
            write!(f, " {name}")?;
        } else {
            write!(f, " <anonymous>")?;
        }

        if let Some(parent) = self.parent() {
            write!(f, " parent {parent}")?;
        }

        Ok(())
    }
}
