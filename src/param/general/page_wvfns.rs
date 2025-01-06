use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

///This keyword controls the paging of wavefunctions to disk in order to save memory. Available options are:
/// - > 0 - all wavefunctions requiring more memory than this value in megabytes will be paged to disk.
/// - 0 - no paging will be performed.
/// - < 0 - all wavefunctions will be paged to disk.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct PageWvfns(i64);

impl Display for PageWvfns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for PageWvfns {
    fn field(&self) -> String {
        "PAGE_WVFNS".to_string()
    }
}

impl From<i64> for PageWvfns {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<i32> for PageWvfns {
    fn from(value: i32) -> Self {
        Self(value as i64)
    }
}

impl From<i16> for PageWvfns {
    fn from(value: i16) -> Self {
        Self(value as i64)
    }
}

impl From<i8> for PageWvfns {
    fn from(value: i8) -> Self {
        Self(value as i64)
    }
}
