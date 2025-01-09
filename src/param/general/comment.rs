
use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    KeywordDisplay,
    Default,
    Serialize,
    Deserialize,
    Hash,
)]
#[keyword_display(field="COMMENT", from=String, borrowed_value=str)]
pub struct Comment(String);
