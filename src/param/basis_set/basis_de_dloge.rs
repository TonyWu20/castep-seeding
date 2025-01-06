use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub struct BasisDeDloge(f64);

impl From<f64> for BasisDeDloge {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Display for BasisDeDloge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for BasisDeDloge {
    fn field(&self) -> String {
        "BASIS_DE_DLOGE".to_string()
    }
}
