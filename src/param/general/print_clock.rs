use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PrintClock(bool);

impl Default for PrintClock {
    fn default() -> Self {
        Self(true)
    }
}

impl Display for PrintClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for PrintClock {
    fn field(&self) -> String {
        "PRINT_CLOCK".to_string()
    }
}

impl From<bool> for PrintClock {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
