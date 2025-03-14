//! Interact and construct the outside world.

pub mod config;
pub mod ui;

use std::{
    num::ParseIntError,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

use num_bigint::{BigInt, BigUint};
use thiserror::Error;

use crate::aux::{Common, Owned};

/// Count of european cents.
#[derive(Owned!)]
pub struct Money(pub Natural);

/// How much two entities owe each other.
#[derive(Owned!)]
pub struct Balance(pub Integer);

/// Natural number (including 0).
pub type Natural = BigUint;
pub type Integer = BigInt;

impl Balance {
    /// Make a negative value positive and
    /// the other way around.
    pub fn flip(&mut self) {
        self.0 *= -1;
    }
}

impl From<Money> for Balance {
    fn from(amount: Money) -> Self {
        Self(amount.0.into())
    }
}

/// Implement noisy calculation traits,
/// delegating to subvalues.
macro_rules! calc {
    (+ $lhs:ty, $rhs:ty $( => $inner:ty )? ) => {
        impl Add<$rhs> for $lhs {
            type Output = Self;
            fn add(self, rhs: $rhs) -> Self {
                Self(self.0 + $(<$inner>::from)?(rhs.0))
            }
        }
    };
    (- $lhs:ty, $rhs:ty $( => $inner:ty )? ) => {
        impl Sub<$rhs> for $lhs {
            type Output = Self;
            fn sub(self, rhs: $rhs) -> Self {
                Self(self.0 - $(<$inner>::from)?(rhs.0))
            }
        }
    };
    (+= $lhs:ty, $rhs:ty ) => {
        impl AddAssign<$rhs> for $lhs {
            fn add_assign(&mut self, rhs: $rhs) {
                *self = self.clone() + rhs;
            }
        }
    };
    (-= $lhs:ty, $rhs:ty ) => {
        impl SubAssign<$rhs> for $lhs {
            fn sub_assign(&mut self, rhs: $rhs) {
                *self = self.clone() - rhs;
            }
        }
    };
}

calc!(+ Money, Money);
calc!(-Money, Money);
calc!(+= Money, Money);
calc!(-= Money, Money);

calc!(+ Balance, Balance);
calc!(-Balance, Balance);
calc!(+= Balance, Balance);
calc!(-= Balance, Balance);

calc!(+ Balance, Money => Integer);
calc!(- Balance, Money => Integer);
calc!(+= Balance, Money);
calc!(-= Balance, Money);

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
