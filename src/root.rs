use castep_cell_io::cell_document::CellDocument;
use castep_periodic_table::{
    data::ELEMENT_TABLE,
    element::{ElementSymbol, LookupElement},
};
use glob::glob;
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
            .into_iter()
            .map(parse_cell_doc_from_path)
            .collect()
    }
    fn get_all_elements(&self) -> Result<HashSet<ElementSymbol>, SeedingErrors> {
        let entries = self.get_cell_entries()?;
        let all_elements: HashSet<ElementSymbol> = HashSet::from_iter(
            entries
                .iter()
                .flat_map(|cell_doc| -> Vec<ElementSymbol> { cell_doc.get_elements() })
                .collect::<Vec<ElementSymbol>>(),
        );
        Ok(all_elements)
    }
    fn fetch_potential_files<P: AsRef<Path>>(
        &self,
        potentials_loc: P,
    ) -> Result<(), SeedingErrors> {
        let elements = self.get_all_elements()?;
        let copy_element = elements.iter().try_for_each(|element| {
            let potential_file = ELEMENT_TABLE.get_by_symbol(*element).potential();
            let potential_path = potentials_loc.as_ref().join(potential_file);
            let dst_path = self.root_path().as_ref().join(potential_file);
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
        match copy_element {
            ControlFlow::Continue(_) => Ok(()),
            ControlFlow::Break(e) => Err(e),
        }
    }
    /// Implementation of generation of struct that impl `SeedFolder`
    fn generate_seed_folders(&self) -> Result<Vec<impl SeedFolder>, SeedingErrors>;
    /// Execute all actions to build seed files from `.cell` files under the root path.
    fn build_all<L: AsRef<Path>, C: CellBuilding, P: ParamBuilding>(
        &self,
        cell_builder: &C,
        param_builder: &P,
        potentials_loc: L,
    ) -> Result<(), SeedingErrors> {
        self.fetch_potential_files(potentials_loc)?;
        self.generate_seed_folders()?
            .iter()
            .try_for_each(|seed| seed.actions(cell_builder, param_builder))
    }
}
