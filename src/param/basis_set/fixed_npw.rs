use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

/// This keyword determines whether a fixed number of plane waves (fixed size
/// basis : TRUE) or a fixed plane wave cutoff energy
/// (fixed quality basis : FALSE) will be used when doing a variable cell
/// calculation.
/// This setting affects geometry optimization with variable cell parameters
/// and molecular dynamics with NPT or NPH ensembles.
/// # Note
/// You should turn off finite basis set correction when using a
/// fixed size basis (see FINITE_BASIS_CORR).
/// # Default
/// `FALSE`
/// # Example
/// `FIXED_NPW : TRUE`
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub struct FixedNPW(bool);

impl From<bool> for FixedNPW {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for FixedNPW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <bool as std::fmt::Display>::fmt(&self.0, f)
    }
}

impl KeywordDisplay for FixedNPW {
    fn field(&self) -> String {
        "FIXED_NPW".to_string()
    }
}
