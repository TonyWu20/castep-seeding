use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

/// This keyword controls the number of points used to estimate the BASIS_DE_DLOGE in automatic calculation of the finite basis set correction. Points are chosen such that the CUT_OFF_ENERGY corresponds to the highest energy in the set of FINITE_BASIS_NPOINTS cutoff energies.
/// # Note
/// This value is only used if FINITE_BASIS_CORR : 2.
/// # Default
/// 3
/// # Example
/// `FINITE_BASIS_NPOINTS : 5`
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FiniteBasisNPoints(u32);

impl Display for FiniteBasisNPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        <u32 as Display>::fmt(&self.0, f)
    }
}

impl From<u32> for FiniteBasisNPoints {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl KeywordDisplay for FiniteBasisNPoints {
    fn field(&self) -> String {
        "FINITE_BASIS_NPOINTS".to_string()
    }
}

impl Default for FiniteBasisNPoints {
    fn default() -> Self {
        Self(3)
    }
}
