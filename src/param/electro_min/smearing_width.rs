use std::fmt::Display;

use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

use crate::param::EnergyUnit;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplay)]
#[keyword_display(direct_display = false, field = "SMEARING_WIDTH")]
/// This keyword determines the width of the Fermi-surface smearing if the system
/// is being treated as a metal.
/// # Note
/// This parameter is used only if FIX_OCCUPANCY : FALSE.
/// # Default
/// 0.2 eV
/// # Example
/// SMEARING_WIDTH : 0.1 eV
pub struct SmearingWidth {
    pub width: f64,
    pub unit: Option<EnergyUnit>,
}

impl Display for SmearingWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.width,
            self.unit.map(|v| v.to_string()).unwrap_or_default()
        )
    }
}
