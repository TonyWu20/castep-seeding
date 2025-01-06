use std::fmt::Display;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::{InvLengthUnit, KeywordDisplay};

/// This keyword determines the maximum size of the g-vectors included in the fine grid.
/// The fine grid is seDefault
/// -1 a0-1 - this results in the fine and standard grids being identical
/// # Example
/// FINE_GMAX : 20 1/angt up such that all g-vectors with |g| less than or equal to FINE_GMAX are included.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Builder)]
#[builder(setter(into), default)]
pub struct FineGMax {
    max: f64,
    unit: InvLengthUnit,
}

impl Display for FineGMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:20.15} {}", self.max, self.unit)
    }
}

impl KeywordDisplay for FineGMax {
    fn field(&self) -> String {
        "FINE_GMAX".to_string()
    }
}

impl Default for FineGMax {
    fn default() -> Self {
        Self {
            max: -1.0,
            unit: InvLengthUnit::Ang,
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
