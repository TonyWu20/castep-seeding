use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
/// This parameter determines the optimization strategy used when there are multiple strategies available for the selected algorithm and they have differing costs in terms of memory usage and performance. Available options are:
/// - Speed - uses the optimization strategy which maximizes performance at the cost of additional memory usage.
/// - Default - uses the optimization strategy that best balances performance and memory usage.
/// - Memory - uses the optimization strategy which minimizes memory usage at a cost of decreased performance.
/// # Default
/// `Default`
/// # Example
/// `OPT_STRATEGY : Memory`
pub enum OptStrategy {
    Speed,
    #[default]
    Default,
    Memory,
}

impl Display for OptStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptStrategy::Speed => f.write_str("Speed"),
            OptStrategy::Default => f.write_str("Default"),
            OptStrategy::Memory => f.write_str("Memory"),
        }
    }
}

impl KeywordDisplay for OptStrategy {
    fn field(&self) -> String {
        "OPT_STRATEGY".to_string()
    }
}
