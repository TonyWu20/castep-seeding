use std::io;

use castep_cell_io::{CellParseError, EnergyCutoffError};
use glob::{GlobError, PatternError};
use thiserror::Error;

use castep_param_io::param::CastepParamBuilderError;

#[derive(Debug, Error)]
pub enum SeedingErrors {
    #[error("The glob match pattern has invalid UTF-8 characters")]
    InvalidPattern,
    #[error("Did not find any matching results")]
    NoMatchingResults,
    #[error("Pattern error during matching cell files: {0}")]
    MatchingCellFiles(#[from] PatternError),
    #[error("Path error during matching pattern: {0}")]
    GlobError(#[from] GlobError),
    #[error("Error during reading given cell file path: {0}")]
    ReadToString(#[from] io::Error),
    #[error("Error during parsing cell file: {0}")]
    CellParseError(#[from] CellParseError),
    #[error("Error during creating seed dir: {0}")]
    CreateSeedDir(io::Error),
    #[error("Error during creating soft link: {0}")]
    SoftlinkError(io::Error),
    #[error("Error during copy: {0}")]
    CopyError(io::Error),
    #[error("Error during writing: {0}")]
    WriteError(io::Error),
    #[error("Directory already exists")]
    DirectoryExist,
    #[error("Error getting cutoff energy: {0}")]
    CutoffEnergy(#[from] EnergyCutoffError),
    #[error("Error in building castep param")]
    ParamBuildingError(#[from] CastepParamBuilderError),
}
