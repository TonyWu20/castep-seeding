use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub struct WriteCheckpoint {
    values: WriteCheckpointValue,
    options: WriteCheckpointOption,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum WriteCheckpointValue {
    None,
    Minimal,
    Both,
    #[default]
    All,
    Full,
}

impl Display for WriteCheckpointValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteCheckpointValue::None => f.write_str("NONE"),
            WriteCheckpointValue::Minimal => f.write_str("Minimal"),
            WriteCheckpointValue::Both => f.write_str("Both"),
            WriteCheckpointValue::All => f.write_str("ALL"),
            WriteCheckpointValue::Full => f.write_str("Full"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WriteCheckpointOption {
    Success(WriteCheckpointValue),
    Failure(WriteCheckpointValue),
    Backup(WriteCheckpointValue),
}

impl Display for WriteCheckpointOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteCheckpointOption::Success(v) => write!(f, "SUCCESS={v}"),
            WriteCheckpointOption::Failure(v) => write!(f, "FAILURE={v}"),
            WriteCheckpointOption::Backup(v) => write!(f, "BACKUP={v}"),
        }
    }
}

impl Default for WriteCheckpointOption {
    fn default() -> Self {
        Self::Success(WriteCheckpointValue::All)
    }
}
