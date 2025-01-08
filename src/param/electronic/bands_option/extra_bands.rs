use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
/// This keyword controls the percentage of extra bands in addition to the number
/// of occupied bands. These extra bands are necessary for metals or finite
/// temperature insulators.
/// # Note
/// It is not possible to have both the NBANDS keyword and either the NEXTRA_BANDS
/// or PERC_EXTRA_BANDS keywords present in the same input file.
pub enum ExtraBands {
    NextraBands(u64),
    PercExtraBands(f64),
}

impl Display for ExtraBands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtraBands::NextraBands(n) => write!(f, "{n}"),
            ExtraBands::PercExtraBands(p) => write!(f, "{p}"),
        }
    }
}

impl KeywordDisplay for ExtraBands {
    fn field(&self) -> String {
        match self {
            ExtraBands::NextraBands(_) => "NEXTRA_BANDS".to_string(),
            ExtraBands::PercExtraBands(_) => "PERC_EXTRA_BANDS".to_string(),
        }
    }
}
