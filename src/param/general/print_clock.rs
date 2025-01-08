use std::fmt::Display;

use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="PRINT_CLOCK",from=bool,value=bool)]
pub struct PrintClock(bool);

impl Default for PrintClock {
    fn default() -> Self {
        Self(true)
    }
}
