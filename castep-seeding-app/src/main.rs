use anyhow::Result;
use castep_cell_io::cell_document::CellDocument;
use castep_param_io::param::CastepParam;
/// Example Usage
use castep_seeding::CellBuilding;
use castep_seeding::ParamBuilding;
use std::env;
use std::path::{Path, PathBuf};

use castep_seeding::{parse_cell_doc_from_path, RootJobs, SeedFolder, SeedingErrors};

struct RootFolder {
    path: PathBuf,
}

impl RootFolder {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().into(),
        }
    }
}

struct Seed<P: AsRef<Path>> {
    cell_path: P,
    cell_doc: CellDocument,
}

impl<P: AsRef<Path>> Seed<P> {
    fn from_cell_path(cell_path: P) -> Result<Self, SeedingErrors> {
        let cell_doc = parse_cell_doc_from_path(&cell_path)?;
        Ok(Self {
            cell_path,
            cell_doc,
        })
    }
}

impl<P> SeedFolder for Seed<P>
where
    P: AsRef<Path>,
{
    fn seed_name(&self) -> &str {
        self.cell_path
            .as_ref()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    }

    fn root_dir(&self) -> impl AsRef<Path> {
        self.cell_path.as_ref().parent().unwrap()
    }

    fn cell_template(&self) -> &CellDocument {
        &self.cell_doc
    }
}

impl RootJobs for RootFolder {
    fn root_path(&self) -> impl AsRef<Path> {
        &self.path
    }

    fn generate_seed_folders(&self) -> Result<Vec<impl SeedFolder>, crate::SeedingErrors> {
        self.get_cell_paths()?
            .into_iter()
            .map(Seed::from_cell_path)
            .collect()
    }
}

struct Configurator {
    use_edft: bool,
    potentials_loc: PathBuf,
}

impl Configurator {
    fn new<P: AsRef<Path>>(use_edft: bool, potentials_loc: P) -> Self {
        Self {
            use_edft,
            potentials_loc: potentials_loc.as_ref().into(),
        }
    }
}

impl CellBuilding for Configurator {}

impl ParamBuilding for Configurator {
    fn build_param_for_task(
        &self,
        template_cell: &CellDocument,
        castep_task: castep_cell_io::CastepTask,
    ) -> Result<CastepParam, SeedingErrors> {
        match castep_task {
            castep_cell_io::CastepTask::BandStructure => self.dos_param_template(
                template_cell,
                castep_cell_io::EnergyCutoff::Ultrafine,
                self.use_edft,
                &self.potentials_loc,
            ),
            castep_cell_io::CastepTask::GeometryOptimization => self.geom_opt_param_template(
                template_cell,
                castep_cell_io::EnergyCutoff::Ultrafine,
                self.use_edft,
                &self.potentials_loc,
            ),
        }
    }
}

fn main() -> Result<()> {
    let cwd = env::current_dir()?;
    let config = Configurator::new(true, &cwd);
    let root_folder = RootFolder::new(&cwd);
    root_folder.build_all(&config, &config, &cwd)?;
    println!("Successfully created seed files for castep");
    Ok(())
}
