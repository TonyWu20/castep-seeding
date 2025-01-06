use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum ContinueReuse {
    Continuation(Continuation),
    Reuse(Reuse),
}

impl Default for ContinueReuse {
    fn default() -> Self {
        Self::Continuation(Continuation::Default)
    }
}

impl From<Continuation> for ContinueReuse {
    fn from(value: Continuation) -> Self {
        ContinueReuse::Continuation(value)
    }
}

impl From<Reuse> for ContinueReuse {
    fn from(value: Reuse) -> Self {
        ContinueReuse::Reuse(value)
    }
}

#[derive(Debug, Clone, Default, Hash, Serialize, Deserialize)]
pub enum Continuation {
    #[default]
    Default,
    File(String),
}

#[derive(Debug, Clone, Default, Hash, Serialize, Deserialize)]
pub enum Reuse {
    #[default]
    Default,
    File(String),
}

impl Display for Continuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Continuation::Default => f.write_str("default"),
            Continuation::File(s) => write!(f, "{s}"),
        }
    }
}

impl KeywordDisplay for Continuation {
    fn field(&self) -> String {
        "CONTINUATION".to_string()
    }
}

impl Display for Reuse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reuse::Default => f.write_str("default"),
            Reuse::File(s) => write!(f, "{s}"),
        }
    }
}

impl Display for ContinueReuse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContinueReuse::Continuation(continuation) => write!(f, "{continuation}"),
            ContinueReuse::Reuse(reuse) => write!(f, "{reuse}"),
        }
    }
}

impl KeywordDisplay for ContinueReuse {
    fn field(&self) -> String {
        match self {
            ContinueReuse::Continuation(_) => "CONTINUATION".to_string(),
            ContinueReuse::Reuse(_) => "REUSE".to_string(),
        }
    }
}
