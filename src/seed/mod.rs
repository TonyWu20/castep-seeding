#![allow(dead_code)]
use std::{fs::read_to_string, path::Path};

use castep_cell_io::{cell_document::CellDocument, CellParser};

use crate::error::SeedingErrors;

pub(crate) fn parse_cell_doc_from_path<P: AsRef<Path>>(
    path: P,
) -> Result<CellDocument, SeedingErrors> {
    let cell_content = read_to_string(&path).map_err(SeedingErrors::ReadToString)?;
    CellParser::from(&cell_content)
        .parse()
        .map_err(SeedingErrors::CellParseError)
}

pub mod seed_folder;
pub mod seed_setup;
