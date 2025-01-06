use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
/// This keyword contains a string which specifies the name of file to which checkpoint (continuation) data are to be written.
/// # Default
/// `seedname.check`
/// # Example
/// `CHECKPOINT : test.check`
pub struct Checkpoint(String);

impl Default for Checkpoint {
    fn default() -> Self {
        Self("test.check".to_string())
    }
}

impl Display for Checkpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for Checkpoint {
    fn field(&self) -> String {
        "CHECKPOINT".to_string()
    }
}
