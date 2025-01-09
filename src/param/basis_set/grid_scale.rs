
use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


/// This keyword determines the size of the standard grid, relative to the diameter of the cutoff sphere.
/// # Default
/// `1.75`
/// # Example
/// `GRID_SCALE : 2.0`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplay)]
#[keyword_display(field="GRID_SCALE",
    from=f64,
    value=f64,
    display_format="{:20.15}"
)]
pub struct GridScale(f64);

impl Default for GridScale {
    fn default() -> Self {
        Self(1.75)
    }
}
