use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

/// This keyword specifies the cutoff energy for the plane wave basis sets that will be used in the calculation.
/// If the BASIS_PRECISION is defined, the cutoff energy will be equal to the highest of the cutoff energies associated with the chosen level of accuracy, for the pseudopotentials used in the calculation.
/// If neither the BASIS_PRECISION nor the CUT_OFF_ENERGY are defined, the default cutoff energy is that associated with the FINE level of accuracy, for the pseudopotentials in the calculation.
/// # Note
/// It is not possible to specify both the BASIS_PRECISION and the CUT_OFF_ENERGY in a single file.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd, Default)]
pub struct CutoffEnergy(f64);

impl Display for CutoffEnergy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:20.15}", self.0)
    }
}

impl KeywordDisplay for CutoffEnergy {
    fn field(&self) -> String {
        "CUT_OFF_ENERGY".to_string()
    }
}

impl From<f64> for CutoffEnergy {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
