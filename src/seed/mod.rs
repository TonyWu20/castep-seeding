#![allow(dead_code)]
use std::{
    fs::{create_dir, read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use castep_cell_io::{CastepParams, CastepTask, CellDocument, CellParser, SeedfileGenerator};
use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};

use crate::error::SeedingErrors;

/// A struct to create seed file folders.
/// Required psuedopotential files must be copied to the root dir first.
#[derive(Debug)]
pub struct Seed {
    seedname: String,
    root_dir: PathBuf,
    cell_doc: CellDocument,
}

pub fn parse_cell_doc_from_path<P: AsRef<Path>>(path: P) -> Result<CellDocument, SeedingErrors> {
    let cell_content = read_to_string(&path).map_err(SeedingErrors::ReadToString)?;
    CellParser::from(&cell_content)
        .parse()
        .map_err(SeedingErrors::CellParseError)
}

pub trait SeedFolder {
    fn seed_dir(&self) -> PathBuf;
    fn create_seed_dir(&self) -> Result<PathBuf, SeedingErrors>;
    fn build_castep_cell(&self, castep_task: CastepTask) -> CellDocument;
    fn create_castep_param(&self, castep_task: CastepTask) -> CastepParams;
    fn soft_link_potentials(&self) -> Result<(), SeedingErrors>;
    fn create_seed_file<P: AsRef<Path>, F: AsRef<[u8]>>(
        &self,
        filename: P,
        file_content: F,
    ) -> Result<(), SeedingErrors>;
    fn write_files(&self) -> Result<(), SeedingErrors>;
    fn actions(&self) -> Result<(), SeedingErrors>;
}

impl SeedFolder for Seed {
    fn seed_dir(&self) -> PathBuf {
        self.root_dir().join(self.seedname())
    }

    fn create_seed_dir(&self) -> Result<PathBuf, SeedingErrors> {
        let seed_dir = self.seed_dir();
        if seed_dir.exists() {
            return Ok(seed_dir);
        }
        create_dir(&seed_dir).map_err(SeedingErrors::CreateSeedDir)?;
        Ok(seed_dir)
    }

    fn create_castep_param(&self, castep_task: CastepTask) -> CastepParams {
        let generator = SeedfileGenerator::new(castep_task, self.cell_doc().clone());
        generator.generate_castep_param(self.root_dir())
    }

    fn build_castep_cell(&self, castep_task: CastepTask) -> CellDocument {
        let generator = SeedfileGenerator::new(castep_task, self.cell_doc().clone());
        generator.generate_cell_file()
    }

    fn soft_link_potentials(&self) -> Result<(), SeedingErrors> {
        self.cell_doc().get_elements().iter().try_for_each(|&elm| {
            let potential_file = ELEMENT_TABLE.get_by_symbol(elm).potential();
            let src_path = self.root_dir().join(potential_file);
            let dst_path = self.seed_dir().join(potential_file);
            #[cfg(target_os = "windows")]
            {
                std::os::windows::fs::symlink_file(src_path, dst_path)
                    .map_err(SeedingErrors::SoftlinkError)
            }
            #[cfg(not(target_os = "windows"))]
            std::os::unix::fs::symlink(src_path, dst_path).map_err(SeedingErrors::SoftlinkError)
        })
    }

    fn create_seed_file<P: AsRef<Path>, F: AsRef<[u8]>>(
        &self,
        filename: P,
        file_content: F,
    ) -> Result<(), SeedingErrors> {
        let seed_dir = self.seed_dir();
        let file_path = seed_dir.join(filename);
        let mut file = File::create(file_path).map_err(SeedingErrors::WriteError)?;
        file.write_all(file_content.as_ref())
            .map_err(SeedingErrors::WriteError)?;
        Ok(())
    }
    fn write_files(&self) -> Result<(), SeedingErrors> {
        self.create_seed_file(
            format!("{}.param", self.seedname()),
            self.create_castep_param(CastepTask::GeometryOptimization)
                .to_string(),
        )?;
        self.create_seed_file(
            format!("{}_DOS.param", self.seedname()),
            self.create_castep_param(CastepTask::BandStructure)
                .to_string(),
        )?;
        self.create_seed_file(
            format!("{}.cell", self.seedname()),
            self.build_castep_cell(CastepTask::GeometryOptimization)
                .to_string(),
        )?;
        self.create_seed_file(
            format!("{}_DOS.cell", self.seedname()),
            self.build_castep_cell(CastepTask::BandStructure)
                .to_string(),
        )?;
        Ok(())
    }

    fn actions(&self) -> Result<(), SeedingErrors> {
        self.create_seed_dir()?;
        self.write_files()?;
        self.soft_link_potentials()
    }
}

impl Seed {
    pub fn seedname(&self) -> &str {
        &self.seedname
    }

    pub fn root_dir(&self) -> &PathBuf {
        &self.root_dir
    }

    pub fn cell_doc(&self) -> &CellDocument {
        &self.cell_doc
    }

    pub fn from_matched_path<P: AsRef<Path>>(path: P) -> Result<Self, SeedingErrors> {
        let seedname = path.as_ref().file_stem().unwrap().to_string_lossy();
        let cell_doc = parse_cell_doc_from_path(&path)?;
        let root_dir = path.as_ref().parent().unwrap().to_owned(); // guaranteed
        Ok(Seed::new(format!("{seedname}"), root_dir, cell_doc))
    }

    pub fn new(seedname: String, root_dir: PathBuf, cell_doc: CellDocument) -> Self {
        Self {
            seedname,
            root_dir,
            cell_doc,
        }
    }
}
