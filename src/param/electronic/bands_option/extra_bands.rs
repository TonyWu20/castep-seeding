use std::fmt::Display;

use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplay)]
/// This keyword controls the percentage of extra bands in addition to the number
/// of occupied bands. These extra bands are necessary for metals or finite
/// temperature insulators.
/// # Note
/// It is not possible to have both the NBANDS keyword and either the NEXTRA_BANDS
/// or PERC_EXTRA_BANDS keywords present in the same input file.
#[keyword_display(specified_fields = true)]
pub enum ExtraBands {
    #[keyword_display(field = "NEXTRA_BANDS")]
    NextraBands(u64),
    #[keyword_display(field = "PERC_EXTRA_BANDS")]
    PercExtraBands(f64),
}
