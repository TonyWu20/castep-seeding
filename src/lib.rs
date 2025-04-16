mod auxiliary;
mod error;
mod export;
mod root;
mod seed;

pub use crate::root::RootJobs;
pub use error::SeedingErrors;
pub use seed::parse_cell_doc_from_path;
pub use seed::seed_folder::SeedFolder;
pub use seed::seed_setup::{CellBuilding, ParamBuilding};

#[cfg(test)]
mod test {
    /// Example Usage
    use crate::CellBuilding;
    use crate::ParamBuilding;
    use castep_cell_io::cell_document::{CellDocument, KpointQuality};
    use castep_param_io::param::CastepParam;
    use std::path::{Path, PathBuf};

    use crate::{
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

    #[test]
    fn execution() {
        let test_folder = RootFolder::new("test/Bi2Te3_001_Fe_2.2");
        let potentials_loc = "/Users/tonywu/Downloads/Potentials";
        let config = Configurator::new(true, KpointQuality::Coarse, potentials_loc);
        match test_folder.build_all(&config, &config, potentials_loc) {
            Ok(()) => {
                println!("Success")
            }
            Err(e) => println!("{e}"),
        }
    }
}
