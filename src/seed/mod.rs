use std::{fs::read_to_string, path::Path};

use castep_cell_fmt::parse;
use castep_cell_io::CellDocument;

use crate::error::SeedingErrors;

pub fn parse_cell_doc_from_path<P: AsRef<Path>>(path: P) -> Result<CellDocument, SeedingErrors> {
    let cell_content = read_to_string(&path).map_err(SeedingErrors::ReadToString)?;
    parse::<CellDocument>(&cell_content).map_err(SeedingErrors::CellParseError)
}

pub mod seed_folder;
pub mod seed_setup;
