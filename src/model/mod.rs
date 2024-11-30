//! High-level structures which can be thrown to the store.
//!
//! This might seem eerily similar to [`crate::entity`], however, that module is *directly*
//! extracted from the database and automatically generated.
//! Essentially, this serves as a more type-safe and less errorful interface to the store
//! — you will probably never need to directly use [`crate::entity`] if you don't want to
//! interface directly with the database.

pub mod db;

use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

pub mod product {
    use super::*;

    /// Specifies a product to buy, optionally also creating a new one instead.
    pub enum Spec {
        Existing(Ref),
        AlsoRegister(New),
    }

    /// References an existing product by its GTIN (basically its barcode) XOR its name.
    pub enum Ref {
        Gtin(Gtin),
        Ident(Ident),
    }

    /// Everything needed in order to register a new product.
    pub struct New {
        /// How this new product can be referred to.
        pub name: Name,
    }

    /// How to name any given product.
    pub enum Name {
        Gtin(Gtin),
        Ident(Ident),
        Both { gtin: Gtin, ident: Ident },
    }
}

/// Global trade item number. The number behind the barcode you find in stores.
///
/// Internationally standardized.
/// This encompasses typical products one would buy off-the-shelf
/// as well ase more specialized cases like books and smaller products.
///
/// # Note on validation
///
/// While there are only limited possibilities for the lengths of GTINs
/// (namely, 8, 10, 13, 14), this is not validated.
/// Any positive number with at most 14 digits in base 10 is accepted.
/// Shorter ones are just padded with zeroes at the start.
///
/// # Resources
///
/// - <https://en.wikipedia.org/wiki/Global_Trade_Item_Number>
// largest number representable by 14 digits is `10^14 - 1`,
// which requires `ceil(log2(10^14 - 1)) = 47` bits
// next largest int is u64
// which has the nice side effect of "automatically" padding shorter GTINs with zeroes
pub struct Gtin(u64);

impl Gtin {
    /// The largest possible GTIN with 14 digits. For now, that is.
    const MAX: Self = Self(10u64.pow(14) - 1);
}

impl FromStr for Gtin {
    type Err = GtinParseError;
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let source = source.parse()?;

        if source > Self::MAX.0 {
            return Err(Self::Err::TooLong { n: digits(source) });
        }

        Ok(Self(source))
    }
}

/// Returns how many digits are in the base 10 repr of `n`.
fn digits(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        n.ilog10() + 1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum GtinParseError {
    #[error("couldn't parse as an integer: {0}")]
    ExpectedInteger(#[from] ParseIntError),
    #[error("contains {n} digits while longest form can only contain 14")]
    TooLong { n: u32 },
}

pub struct Ident(pub String);
pub struct Price(pub u32);
