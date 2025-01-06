use std::fmt::{Display, Write};

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum ChargeUnit {
    #[default]
    E,
    C,
}

impl Display for ChargeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChargeUnit::E => f.write_char('e'),
            ChargeUnit::C => f.write_char('c'),
        }
    }
}

impl KeywordDisplay for ChargeUnit {
    fn field(&self) -> String {
        "CHARGE_UNIT".to_string()
    }
}
