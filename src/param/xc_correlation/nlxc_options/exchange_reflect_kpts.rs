use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct ExchangeReflectKpts(bool);

impl Default for ExchangeReflectKpts {
    fn default() -> Self {
        Self(true)
    }
}

impl From<bool> for ExchangeReflectKpts {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Display for ExchangeReflectKpts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for ExchangeReflectKpts {
    fn field(&self) -> String {
        "NLXC_EXCHANGE_REFLECT_KPTS".to_string()
    }
}
