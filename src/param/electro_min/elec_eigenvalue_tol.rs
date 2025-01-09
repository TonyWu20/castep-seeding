use std::fmt::Display;

use castep_seeding_derive::KeywordDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::{EnergyUnit, Nbands};

use super::ElecEnergyTol;

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd, Builder, KeywordDisplay,
)]
#[builder(setter(into), default)]
#[keyword_display(field = "ELEC_EIGENVALUE_TOL", direct_display = false)]
/// This keyword controls the tolerance for accepting convergence of a single
/// eigenvalue during density mixing minimization.
/// The difference between maximum and minimum eigenvalues over ELEC_CONVERGENCE_WIN
/// iterations must be less than this value.
/// # Default
/// The default value is the lower of 1x10-6 eV and ELEC_ENERGY_TOL*NATOMS/NBANDS, where NATOMS is the total number of atoms in the unit cell.
pub struct ElecEigenvalueTol {
    pub tol: f64,
    pub unit: Option<EnergyUnit>,
}

impl Default for ElecEigenvalueTol {
    fn default() -> Self {
        Self {
            tol: 1e-6,
            unit: Default::default(),
        }
    }
}

impl Display for ElecEigenvalueTol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:20.15} {}",
            self.tol,
            self.unit.map(|v| v.to_string()).unwrap_or_default()
        )
    }
}

impl ElecEigenvalueTol {
    fn default_value(elec_energy_tol: ElecEnergyTol, natoms: usize, nbands: Nbands) -> Self {
        let tol = elec_energy_tol.tol * natoms as f64 / nbands.value() as f64;
        Self {
            tol,
            unit: elec_energy_tol.unit,
        }
    }
}

impl From<f64> for ElecEigenvalueTol {
    fn from(value: f64) -> Self {
        Self {
            tol: value,
            ..Default::default()
        }
    }
}
