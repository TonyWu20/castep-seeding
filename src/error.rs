use std::{error::Error, fmt::Display, io};

use castep_cell_io::CellParseError;
use glob::{GlobError, PatternError};

#[derive(Debug)]
pub enum SeedingErrors {
    InvalidPattern,
    MatchingCellFiles(PatternError),
    GlobError(GlobError),
    ReadToString(io::Error),
    CellParseError(CellParseError),
    CreateSeedDir(io::Error),
    SoftlinkError(io::Error),
    CopyError(io::Error),
    WriteError(io::Error),
    DirectoryExist,
}

impl Display for SeedingErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeedingErrors::MatchingCellFiles(e) => {
                write!(f, "Pattern error during matching cell files: {}", e)
            }
            SeedingErrors::GlobError(e) => {
                write!(f, "Path error during matching pattern: {}", e)
            }
            SeedingErrors::ReadToString(e) => {
                write!(f, "Error during reading given cell file path: {}", e)
            }
            SeedingErrors::CellParseError(e) => {
                write!(f, "Error during parsing cell file: {}", e)
            }
            SeedingErrors::InvalidPattern => {
                write!(f, "The glob match pattern has invalid UTF-8 characters")
            }
            SeedingErrors::CreateSeedDir(e) => {
                write!(f, "Error during creating seed dir: {}", e)
            }
            SeedingErrors::SoftlinkError(e) => {
                write!(f, "Error during creating soft link: {}", e)
            }
            SeedingErrors::CopyError(e) => {
                write!(f, "Error during copying psuedopotential files: {}", e)
            }
            SeedingErrors::WriteError(e) => {
                write!(f, "Error during writing file: {}", e)
            }
            SeedingErrors::DirectoryExist => {
                write!(f, "Directory already exists")
            }
        }
    }
}

impl Error for SeedingErrors {}
