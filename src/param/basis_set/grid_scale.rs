use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

/// This keyword determines the size of the standard grid, relative to the diameter of the cutoff sphere.
/// # Default
/// `1.75`
/// # Example
/// `GRID_SCALE : 2.0`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GridScale(f64);

impl Default for GridScale {
    fn default() -> Self {
        Self(1.75)
    }
}

impl std::fmt::Display for GridScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:20.15}", self.0)
    }
}

impl From<f64> for GridScale {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl KeywordDisplay for GridScale {
    fn field(&self) -> String {
        "GRID_SCALE".to_string()
    }
}
