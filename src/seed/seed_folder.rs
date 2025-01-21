use std::{
    fs::{create_dir, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{error::SeedingErrors, CellBuilding, ParamBuilding};
use castep_cell_io::{cell_document::CellDocument, CastepTask};
use castep_param_io::param::CastepParam;
use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};

/// A trait of how to create seed file folders.
/// Required psuedopotential files must be copied to the root dir first.
pub trait SeedFolder {
    fn seed_name(&self) -> &str;
    fn root_dir(&self) -> impl AsRef<Path>;
    fn cell_template(&self) -> &CellDocument;
    /// Join seed name after the root dir as the seed directory.
    /// You might implement this by yourself to customize the seed directory naming logic.
    fn seed_dir(&self) -> impl AsRef<Path> {
        self.root_dir().as_ref().join(self.seed_name())
    }
    fn create_seed_dir(&self) -> Result<PathBuf, SeedingErrors> {
        let seed_dir = self.seed_dir();
        if seed_dir.as_ref().exists() {
            return Ok(seed_dir.as_ref().into());
        }
        create_dir(&seed_dir).map_err(SeedingErrors::CreateSeedDir)?;
        Ok(seed_dir.as_ref().into())
    }
    fn build_castep_cell(
        &self,
        castep_task: CastepTask,
        builder: &impl CellBuilding,
    ) -> CellDocument {
        builder.build_cell_for_task(self.cell_template(), castep_task)
    }
    fn create_castep_param(
        &self,
        castep_task: CastepTask,
        builder: &impl ParamBuilding,
    ) -> Result<CastepParam, SeedingErrors> {
        builder.build_param_for_task(self.cell_template(), castep_task)
    }
    fn soft_link_potentials<P: AsRef<Path>>(&self, potential_src: P) -> Result<(), SeedingErrors> {
        self.cell_template()
            .get_elements()
            .iter()
            .try_for_each(|&elm| {
                let potential_file = ELEMENT_TABLE.get_by_symbol(elm).potential();
                let src_path = potential_src.as_ref().join(potential_file);
                let dst_path = self.seed_dir().as_ref().join(potential_file);
                dbg!(&src_path);
                dbg!(&dst_path);
                if dst_path.is_symlink() || dst_path.exists() {
                    Ok(())
                } else {
                    #[cfg(target_os = "windows")]
                    {
                        std::os::windows::fs::symlink_file(src_path, dst_path)
                            .map_err(SeedingErrors::SoftlinkError)
                    }
                    #[cfg(not(target_os = "windows"))]
                    std::os::unix::fs::symlink(src_path, dst_path)
                        .map_err(SeedingErrors::SoftlinkError)
                }
            })
    }
    fn create_seed_file<P: AsRef<Path>, F: AsRef<[u8]>>(
        &self,
        filename: P,
        file_content: F,
    ) -> Result<(), SeedingErrors> {
        let seed_dir = self.seed_dir();
        let file_path = seed_dir.as_ref().join(filename);
        let mut file = File::create(file_path).map_err(SeedingErrors::WriteError)?;
        file.write_all(file_content.as_ref())
            .map_err(SeedingErrors::WriteError)?;
        Ok(())
    }
    /// Here generates and writes all files needed.
    /// Implement by yourself to change the behavior.
    fn write_files(
        &self,
        cell_builder: &impl CellBuilding,
        param_builder: &impl ParamBuilding,
    ) -> Result<(), SeedingErrors> {
        self.create_seed_file(
            format!("{}.param", self.seed_name()),
            self.create_castep_param(CastepTask::GeometryOptimization, param_builder)?
                .to_string(),
        )?;
        self.create_seed_file(
            format!("{}_DOS.param", self.seed_name()),
            self.create_castep_param(CastepTask::BandStructure, param_builder)?
                .to_string(),
        )?;
        self.create_seed_file(
            format!("{}.cell", self.seed_name()),
            self.build_castep_cell(CastepTask::GeometryOptimization, cell_builder)
                .to_string(),
        )?;
        self.create_seed_file(
            format!("{}_DOS.cell", self.seed_name()),
            self.build_castep_cell(CastepTask::BandStructure, cell_builder)
                .to_string(),
        )?;
        Ok(())
    }
    /// One command to do all
    fn actions<P: AsRef<Path>>(
        &self,
        cell_builder: &impl CellBuilding,
        param_builder: &impl ParamBuilding,
        potential_src: P,
    ) -> Result<(), SeedingErrors> {
        self.create_seed_dir()?;
        self.write_files(cell_builder, param_builder)?;
        self.soft_link_potentials(potential_src)
    }
}
