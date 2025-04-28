use castep_cell_io::cell_document::{
    sections::species_characters::{SpeciesBlock, SpeciesEntry},
    CellDocument, CellEntries,
};
use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};
use glob::glob;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;
use std::{
    collections::HashSet,
    ops::ControlFlow,
    path::{Path, PathBuf},
};

use crate::{
    error::SeedingErrors, seed::parse_cell_doc_from_path, CellBuilding, ParamBuilding, SeedFolder,
};

/// A trait to process a folder of many cell files.
pub trait RootJobs {
    fn root_path(&self) -> impl AsRef<Path>;
    fn get_cell_paths(&self) -> Result<Vec<PathBuf>, SeedingErrors> {
        let binding = self.root_path().as_ref().join("*.cell");
        let pattern = binding.to_str().ok_or(SeedingErrors::InvalidPattern)?;
        let cell_paths = glob(pattern)
            .map_err(SeedingErrors::MatchingCellFiles)?
            .map(|paths| paths.map_err(SeedingErrors::GlobError))
            .collect::<Result<Vec<PathBuf>, SeedingErrors>>()?;
        if cell_paths.is_empty() {
            Err(SeedingErrors::NoMatchingResults)
        } else {
            Ok(cell_paths)
        }
    }
    fn get_cell_entries(&self) -> Result<Vec<CellDocument>, SeedingErrors> {
        self.get_cell_paths()?
            .par_iter()
            .progress_with_style(ProgressStyle::default_bar())
            .map(parse_cell_doc_from_path)
            .collect()
    }
    fn fetch_potential_files<P: AsRef<Path>>(
        &self,
        potentials_loc: P,
    ) -> Result<(), SeedingErrors> {
        let potential_files: HashSet<String> =
            HashSet::from_iter(self.get_cell_entries()?.iter().flat_map(
                |cell_doc| -> Vec<String> {
                    let species_pots = cell_doc.other_entries().and_then(|v| {
                        v.iter()
                            .find(|entry| matches!(entry, CellEntries::SpeciesPot(_sp)))
                            .and_then(|entry| {
                                if let CellEntries::SpeciesPot(sp) = entry {
                                    Some(
                                        sp.items()
                                            .iter()
                                            .map(|s| s.item())
                                            .cloned()
                                            .collect::<Vec<String>>(),
                                    )
                                } else {
                                    None
                                }
                            })
                    });
                    species_pots.unwrap_or_else(|| {
                        cell_doc
                            .get_elements()
                            .iter()
                            .map(|elm| ELEMENT_TABLE.get_by_symbol(*elm).potential().into())
                            .collect::<Vec<String>>()
                    })
                },
            ));
        let copy_potentials = potential_files.iter().try_for_each(|pot_file| {
            let potential_path = potentials_loc.as_ref().join(pot_file);
            let dst_path = self.root_path().as_ref().join(pot_file);
            if !dst_path.exists() {
                let copy =
                    std::fs::copy(potential_path, dst_path).map_err(SeedingErrors::CopyError);
                match copy {
                    Ok(_) => ControlFlow::Continue(()),
                    Err(e) => ControlFlow::Break(e),
                }
            } else {
                ControlFlow::Continue(())
            }
        });
        match copy_potentials {
            ControlFlow::Continue(_) => Ok(()),
            ControlFlow::Break(e) => Err(e),
        }
    }
    /// Implementation of generation of struct that impl `SeedFolder`
    fn generate_seed_folders(&self) -> Result<Vec<impl SeedFolder + Sync>, SeedingErrors>;
    /// Execute all actions to build seed files from `.cell` files under the root path.
    fn build_all<
        L: AsRef<Path> + Sync,
        C: CellBuilding + Sync,
        P: ParamBuilding + std::marker::Sync,
    >(
        &self,
        cell_builder: &C,
        param_builder: &P,
        potentials_loc: L,
    ) -> Result<(), SeedingErrors> {
        self.fetch_potential_files(&potentials_loc)?;
        self.generate_seed_folders()
            .unwrap()
            .par_iter()
            .progress_with_style(ProgressStyle::default_bar())
            .try_for_each(|seed| seed.actions(cell_builder, param_builder, &potentials_loc))
    }
}
