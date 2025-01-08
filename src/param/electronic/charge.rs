use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

/// Description
/// This keyword specifies the total charge of the system. It can be used instead of `NELECTRONS`.
/// It is not possible to specify the `NELECTRONS`, `NUP`, or `NDOWN` keywords when the `CHARGE` keyword is specified.
/// Default
/// 0
/// Example
/// CHARGE : 3
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Charge(f64);

impl From<f64> for Charge {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Display for Charge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for Charge {
    fn field(&self) -> String {
        "CHARGE".to_string()
    }
}
