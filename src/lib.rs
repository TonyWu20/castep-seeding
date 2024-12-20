use std::path::Path;

use castep_cell_io::{CellBuilding, ParamBuilding};
pub use error::SeedingErrors;

pub use crate::root::RootJobs;

mod auxiliary;
mod error;
mod export;
mod param;
mod root;
mod seed;

pub use seed::seed_folder::SeedFolder;

pub fn execute<R: RootJobs, Q: AsRef<Path>>(
    root: &R,
    cell_builder: &impl CellBuilding,
    param_builder: &impl ParamBuilding,
    potentials_loc: Q,
) -> Result<(), SeedingErrors> {
    root.fetch_potential_files(potentials_loc)?;
    let seed_folders = root.generate_seed_folders()?;
    seed_folders
        .iter()
        .try_for_each(|seed| seed.actions(cell_builder, param_builder))
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use castep_cell_io::{
        CastepParams, CellBuilding, CellDocument, EnergyCutoffError, KpointQuality, ParamBuilding,
    };

    use crate::{
        execute,
        seed::{parse_cell_doc_from_path, seed_folder::SeedFolder},
        RootJobs, SeedingErrors,
    };

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

        fn cell_template(&self) -> &castep_cell_io::CellDocument {
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
        kpoint_quality: KpointQuality,
        potentials_loc: PathBuf,
    }

    impl Configurator {
        fn new<P: AsRef<Path>>(
            use_edft: bool,
            kpoint_quality: KpointQuality,
            potentials_loc: P,
        ) -> Self {
            Self {
                use_edft,
                kpoint_quality,
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
        ) -> Result<CastepParams, EnergyCutoffError> {
            match castep_task {
                castep_cell_io::CastepTask::BandStructure => self.bs_param_template(
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

    #[test]
    fn execution() {
        let test_folder = RootFolder::new("test/Bi2Te3_001_Fe_2.2");
        let potentials_loc = "/Users/tonywu/Downloads/Potentials";
        let config = Configurator::new(true, KpointQuality::Coarse, potentials_loc);
        match execute(&test_folder, &config, &config, potentials_loc) {
            Ok(()) => {
                println!("Success")
            }
            Err(e) => println!("{e}"),
        }
    }
}
