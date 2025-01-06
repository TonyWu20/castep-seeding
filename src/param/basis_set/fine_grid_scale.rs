use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

/// This keyword determines the maximum size of the g-vectors included in the fine grid relative to the standard grid.
/// # Default
/// 1  - this results in the fine and standard grids being identical
/// # Example
/// `FINE_GRID_SCALE : 2.0`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FineGridScale(f64);

impl Display for FineGridScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:20.15}", self.0)
    }
}

impl From<f64> for FineGridScale {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Default for FineGridScale {
    fn default() -> Self {
        Self(1.0)
    }
}

impl KeywordDisplay for FineGridScale {
    fn field(&self) -> String {
        "FINE_GRID_SCALE".to_string()
    }
}
