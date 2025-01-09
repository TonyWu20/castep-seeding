
use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


/// This keyword determines the maximum size of the g-vectors included in the fine grid relative to the standard grid.
/// # Default
/// 1  - this results in the fine and standard grids being identical
/// # Example
/// `FINE_GRID_SCALE : 2.0`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplay)]
#[keyword_display(display_format="{:20.15}", from=f64,value=f64, field="FINE_GRID_SCALE")]
pub struct FineGridScale(f64);

impl Default for FineGridScale {
    fn default() -> Self {
        Self(1.0)
    }
}
