use std::path::Path;

pub use error::SeedingErrors;

pub use crate::{
    root::{RootFolder, RootJobs},
    seed::SeedFolder,
};

mod auxiliary;
mod error;
mod export;
mod param;
mod root;
mod seed;

pub use seed::Seed;

pub fn execute<P: AsRef<Path>, Q: AsRef<Path>>(
    target_dir: P,
    potentials_loc: Q,
) -> Result<(), SeedingErrors> {
    let root = RootFolder::new(target_dir.as_ref().to_path_buf());
    root.fetch_potential_files(potentials_loc)?;
    let seed_folders = root.generate_seed_folders()?;
    seed_folders.iter().try_for_each(|seed| seed.actions())
}

#[cfg(test)]
mod test {
    use crate::execute;

    #[test]
    fn execution() {
        let test_folder = "g-C3N4_Cu_2.2";
        let potentials_loc = "/Users/tonywu/Downloads/Potentials";
        match execute(test_folder, potentials_loc) {
            Ok(_) => {
                println!("Success")
            }
            Err(e) => println!("{e}"),
        }
    }
}
