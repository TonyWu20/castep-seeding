
use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, KeywordDisplay,
)]
#[keyword_display(field="PRINT_MEMORY_USAGE",from=bool,value=bool)]
pub struct PrintMemoryUsage(bool);

impl Default for PrintMemoryUsage {
    fn default() -> Self {
        Self(true)
    }
}
