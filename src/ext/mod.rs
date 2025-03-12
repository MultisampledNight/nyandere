//! Interact and construct the outside world.

pub mod config;
pub mod ui;

use std::{num::ParseIntError, str::FromStr};

use num_bigint::BigUint;
use thiserror::Error;

use crate::aux::{Common, Owned};

/// Count of european cents.
#[derive(Owned!)]
pub struct Money(pub Natural);

/// Natural number (including 0).
pub type Natural = BigUint;

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
#[derive(Common!)]
pub struct Gtin(u64);

impl Gtin {
    /// The largest possible GTIN with 14 digits. For now, that is.
    const MAX: Self = Self(10u64.pow(14) - 1);

    /// Interpret the integer as-is as GTIN.
    ///
    /// # Errors
    ///
    /// Returns an error if the integer is longer than 14 digits.
    pub fn new(source: u64) -> Result<Self, TooLongError> {
        if source > Self::MAX.0 {
            return Err(TooLongError {
                orig: source,
                n: digits(source),
            });
        }

        Ok(Self(source))
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

impl FromStr for Gtin {
    type Err = GtinParseError;
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let source = source.parse()?;
        Self::new(source).map_err(GtinParseError::TooLong)
    }
}

/// Returns how many digits are in the base 10 repr of `n`.
fn digits(n: u64) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum GtinParseError {
    #[error("couldn't parse as an integer: {0}")]
    ExpectedInteger(#[from] ParseIntError),
    #[error("valid int, but too long: {0}")]
    TooLong(TooLongError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("`{orig}` contains {n} digits while longest form can only contain 14")]
pub struct TooLongError {
    pub orig: u64,
    pub n: u32,
}
