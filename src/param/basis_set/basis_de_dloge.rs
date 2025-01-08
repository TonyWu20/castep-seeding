use std::fmt::Display;

use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Default, KeywordDisplay,
)]
#[keyword_display(from=f64, value=f64, field="BASIS_DE_DLOGE")]
pub struct BasisDeDloge(f64);
