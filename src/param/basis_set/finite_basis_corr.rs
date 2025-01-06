use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

/// This keyword determines whether or not to apply a finite basis set correction to energy and stress when cell parameters change.
/// Available options are:
/// - 0 - no correction.
/// - 1 - manual correction using specified BASIS_DE_DLOGE.
/// - 2 - automatic correction using FINITE_BASIS_NPOINTS and FINITE_BASIS_SPACING.
/// # Note
/// - If FINITE_BASIS_CORR : 1, a value for BASIS_DE_DLOGE must be supplied.
/// - You should turn off finite basis set correction when using a fixed size basis (see `FixedNPW`).
#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default,
)]
pub enum FiniteBasisCorr {
    NoCorrection,
    Manual,
    #[default]
    Auto,
}

impl Display for FiniteBasisCorr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FiniteBasisCorr::NoCorrection => f.write_str("0"),
            FiniteBasisCorr::Manual => f.write_str("1"),
            FiniteBasisCorr::Auto => f.write_str("2"),
        }
    }
}

impl KeywordDisplay for FiniteBasisCorr {
    fn field(&self) -> String {
        "FINITE_BASIS_CORR".to_string()
    }
}
