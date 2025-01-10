use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword specifies whether or not to perform a population analysis on the
/// final ground state of the calculation.
/// # Default
/// TRUE
/// # Example
/// `POPN_CALCULATE : FALSE`
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="POPN_CALCULATE", from=bool,value=bool)]
pub struct PopnCalculate(bool);

impl Default for PopnCalculate {
    fn default() -> Self {
        Self(true)
    }
}
