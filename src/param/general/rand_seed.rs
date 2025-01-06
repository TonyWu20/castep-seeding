use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default, Hash,
)]
pub struct RandSeed(i64);

impl Display for RandSeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for RandSeed {
    fn field(&self) -> String {
        "RAND_SEED".to_string()
    }
}

impl From<i64> for RandSeed {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
