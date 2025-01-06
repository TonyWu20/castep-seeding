use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrintMemoryUsage(bool);

impl Default for PrintMemoryUsage {
    fn default() -> Self {
        Self(true)
    }
}

impl Display for PrintMemoryUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for PrintMemoryUsage {
    fn field(&self) -> String {
        "PRINT_MEMORY_USAGE".to_string()
    }
}

impl From<bool> for PrintMemoryUsage {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
