use std::fmt::Display;

use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, KeywordDisplay,
)]
#[keyword_display(borrowed_value=str, field="CHECKPOINT", from=String)]
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
