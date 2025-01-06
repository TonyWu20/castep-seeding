use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BackUpSetting {
    /// This keyword specifies the interval, in seconds, between updates of the backup restart files. This keyword is applicable for geometry optimization, molecular dynamics, phonon or transition state search runs. A value which is less than or equal to zero indicates that no updates will be performed.
    BackupInterval(i64),
    /// This keyword specifies the number of geometry optimization or molecular dynamics iterations between updates of the backup restart files.
    NumBackupIter(u64),
}

impl Default for BackUpSetting {
    fn default() -> Self {
        Self::NumBackupIter(5)
    }
}

impl Display for BackUpSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackUpSetting::BackupInterval(i) => write!(f, "{i}"),
            BackUpSetting::NumBackupIter(u) => write!(f, "{u}"),
        }
    }
}

impl KeywordDisplay for BackUpSetting {
    fn field(&self) -> String {
        match self {
            BackUpSetting::BackupInterval(_) => "BACKUP_INTERVAL".to_string(),
            BackUpSetting::NumBackupIter(_) => "NUM_BACKUP_ITER".to_string(),
        }
    }
}
