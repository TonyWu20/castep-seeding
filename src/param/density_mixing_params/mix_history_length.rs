use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword determines the maximum number of densities to store in the
/// history used during the density mixing procedure.
/// # Default
/// 7
/// Example
/// `MIX_HISTORY_LENGTH : 5`
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="MIX_HISTORY_LENGTH", from=u32,value=u32)]
pub struct MixHistoryLength(u32);

impl Default for MixHistoryLength {
    fn default() -> Self {
        Self(7)
    }
}
