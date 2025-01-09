use std::fmt::Display;

use castep_seeding_derive::KeywordDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::InvLengthUnit;

/// This keyword determines the maximum size of the g-vectors included in the fine grid.
/// The fine grid is seDefault
/// -1 a0-1 - this results in the fine and standard grids being identical
/// # Example
/// FINE_GMAX : 20 1/angt up such that all g-vectors with |g| less than or equal to FINE_GMAX are included.
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Builder, KeywordDisplay,
)]
#[builder(setter(into), default)]
#[keyword_display(direct_display = false, field = "FINE_GMAX")]
pub struct FineGMax {
    pub max: f64,
    pub unit: Option<InvLengthUnit>,
}

impl Display for FineGMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:20.15} {}",
            self.max,
            self.unit.map(|v| v.to_string()).unwrap_or_default()
        )
    }
}

impl Default for FineGMax {
    fn default() -> Self {
        Self {
            max: -1.0,
            unit: None,
        }
    }
}

impl From<f64> for FineGMax {
    fn from(value: f64) -> Self {
        Self {
            max: value,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::FineGMaxBuilder;

    fn fine_gmax() {
        let f = FineGMaxBuilder::default().max(20).build().unwrap();
        println!("{f}");
    }
}
