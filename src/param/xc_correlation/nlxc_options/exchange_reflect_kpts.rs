use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, KeywordDisplay,
)]
#[keyword_display(field="NLXC_EXCHANGE_REFLECT_KPTS", from=bool,value=bool)]
pub struct ExchangeReflectKpts(bool);

impl Default for ExchangeReflectKpts {
    fn default() -> Self {
        Self(true)
    }
}
