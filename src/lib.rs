mod auxiliary;
pub mod cell_doc;
pub mod cutoff;
mod error;
mod export;
pub mod param;
mod root;
mod seed;

pub use cell_doc::{cell_elements, cell_to_string, total_spin};
pub use cutoff::{EnergyCutoff, EnergyCutoffError};
pub use error::SeedingErrors;
pub use param::{CastepParam, CastepTask};
pub use root::RootJobs;
pub use seed::parse_cell_doc_from_path;
pub use seed::seed_folder::SeedFolder;
pub use seed::seed_setup::{CellBuilding, ParamBuilding};

#[cfg(test)]
mod test {
    /// Example Usage
    use crate::{
        param::{CastepParam, CastepTask},
        seed::{parse_cell_doc_from_path, seed_folder::SeedFolder},
        CellBuilding, EnergyCutoff, ParamBuilding, RootJobs, SeedingErrors,
    };
    use castep_cell_io::CellDocument;
    use std::path::{Path, PathBuf};

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
        P: AsRef<Path> + std::marker::Send,
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
            castep_task: CastepTask,
        ) -> Result<CastepParam, SeedingErrors> {
            match castep_task {
                CastepTask::BandStructure => self.dos_param_template(
                    template_cell,
                    EnergyCutoff::Ultrafine,
                    self.use_edft,
                    &self.potentials_loc,
                ),
                CastepTask::GeometryOptimization => self.geom_opt_param_template(
                    template_cell,
                    EnergyCutoff::Ultrafine,
                    self.use_edft,
                    &self.potentials_loc,
                ),
                _ => unreachable!("only BandStructure and GeometryOptimization are handled"),
            }
        }
    }

    #[test]
    fn execution() {
        let test_folder = RootFolder::new("test/Bi2Te3_001_Fe_2.2");
        let potentials_loc = "/Users/tonywu/Downloads/Potentials";
        let config = Configurator::new(true, potentials_loc);
        match test_folder.build_all(&config, &config, potentials_loc) {
            Ok(()) => {
                println!("Success")
            }
            Err(e) => println!("{e}"),
        }
    }
}
