use std::fmt::Display;

use castep_seeding_derive::KeywordDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::{EnergyUnit, KeywordDisplay};
/// This keyword determines the spacing of cutoff energies used to estimate
/// the `BasisDeDloge` in automatic calculation of the finite basis set correction.
/// Points are chosen such that the `CutoffEnergy` corresponds to the highest energy
/// in the set of `FiniteBasisNPoints` cutoff energies.
/// # Note
/// This value is only used if FINITE_BASIS_CORR : 2.
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Builder, KeywordDisplay,
)]
#[builder(setter(into), default)]
#[keyword_display(field = "FINITE_BASIS_SPACING", direct_display = false)]
pub struct FiniteBasisSpacing {
    spacing: f64,
    unit: Option<EnergyUnit>,
}

impl Default for FiniteBasisSpacing {
    fn default() -> Self {
        Self {
            spacing: 5.0,
            unit: None,
        }
    }
}

impl Display for FiniteBasisSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:20.15} {}",
            self.spacing,
            self.unit.map(|v| v.to_string()).unwrap_or_default()
        )
    }
}
