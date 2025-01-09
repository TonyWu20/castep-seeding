use std::fmt::Display;

use castep_seeding_derive::KeywordDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::EnergyUnit;

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, KeywordDisplay, Builder,
)]
#[keyword_display(field = "ELEC_ENERGY_TOL", direct_display = false)]
#[builder(setter(into), default)]
/// This keyword controls the tolerance for accepting convergence of the total
/// energy in an electronic minimization.
/// # Note
/// The difference between maximum and minimum energies over ELEC_CONVERGENCE_WIN
/// iterations must be less than this value.
/// # Default
/// 1x10-5 eV per atom
pub struct ElecEnergyTol {
    pub tol: f64,
    pub unit: Option<EnergyUnit>,
}

impl Display for ElecEnergyTol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:20.15e} {}",
            self.tol,
            self.unit.map(|v| v.to_string()).unwrap_or_default()
        )
    }
}

impl Default for ElecEnergyTol {
    fn default() -> Self {
        Self {
            tol: 1e-5,
            unit: None,
        }
    }
}

impl From<f64> for ElecEnergyTol {
    fn from(value: f64) -> Self {
        Self {
            tol: value,
            ..Default::default()
        }
    }
}
