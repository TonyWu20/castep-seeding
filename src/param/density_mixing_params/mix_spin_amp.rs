use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword determines the mixing amplitude for the spin density in the density
/// mixing procedure.
/// # Default
/// 2.0
/// Example
/// `MIX_SPIN_AMP : 1.754`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplay)]
#[keyword_display(field="MIX_SPIN_AMP", from=f64,value=f64, display_format="{:20.15}")]
pub struct MixSpinAmp(f64);

impl Default for MixSpinAmp {
    fn default() -> Self {
        Self(2.0)
    }
}
