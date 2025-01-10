use castep_seeding_derive::KeywordDisplayStruct;
use serde::{Deserialize, Serialize};

use crate::param::InvLengthUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplayStruct,
)]
#[keyword_display(field = "MIX_SPIN_GMAX", display_format = "{:20.15} {}")]
/// This keyword determines the maximum g-vector at which the spin density is
/// mixed in the density mixing procedure.
/// # Default
/// 1.5 Å-1
/// # Example
/// `MIX_SPIN_GMAX : 0.89 1/ang`
pub struct MixSpinGmax {
    pub gmax: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<InvLengthUnit>,
}

impl Default for MixSpinGmax {
    fn default() -> Self {
        Self {
            gmax: 1.5,
            unit: Default::default(),
        }
    }
}
