use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

/// This keyword specifies the maximum run time for the job, in seconds. If the RUN_TIME is greater than zero, the job will exit cleanly before the specified time has elapsed, leaving as little unused time as possible.
/// If RUN_TIME is less than or equal to zero, no time limit will be imposed on the run.
/// # Default
/// 0
/// # Example
/// `RUN_TIME : 360`
#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct RunTime(i64);

impl Display for RunTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for RunTime {
    fn field(&self) -> String {
        "RUN_TIME".to_string()
    }
}

impl From<i64> for RunTime {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
