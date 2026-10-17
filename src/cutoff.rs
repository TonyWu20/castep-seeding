use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use thiserror::Error;

/// The pseudopotential cutoff quality presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EnergyCutoff {
    Coarse,
    Medium,
    Fine,
    #[default]
    Ultrafine,
}

#[derive(Error, Debug)]
pub enum EnergyCutoffError {
    #[error("open potential file failed. {0}")]
    OpenPotentialFile(#[from] io::Error),
}

impl EnergyCutoff {
    /// Read the recommended cutoff energy out of a pseudopotential file.
    ///
    /// The file carries `COARSE`, `MEDIUM`, and `FINE` keyword lines with
    /// integer cutoff values. The value is rounded up to the next multiple
    /// of ten. `Ultrafine` multiplies the `FINE` value by 1.1 before
    /// rounding.
    pub fn get_cutoff_energy_from_pot<P: AsRef<Path>>(
        &self,
        potential_file: P,
    ) -> Result<f64, EnergyCutoffError> {
        let file = File::open(potential_file.as_ref())?;
        let reader = BufReader::new(file);
        let keyword = match self {
            EnergyCutoff::Coarse => "COARSE",
            EnergyCutoff::Medium => "MEDIUM",
            EnergyCutoff::Fine | EnergyCutoff::Ultrafine => "FINE",
        };
        let cutoff_energy = reader
            .lines()
            .find(|line| line.as_ref().unwrap().contains(keyword))
            .map(|line| {
                // This pattern does not handle `otfg` file
                // Support in future
                let num_str = line.as_ref().unwrap().split_whitespace().next().unwrap();
                num_str.parse::<u32>().expect("Can't parse into `u32`")
            })
            .expect("Failed to parse fine energy from pseudopotential file.");
        let round_bigger_tenth = |num: u32| -> f64 {
            match num % 10 {
                0 => num as f64,
                _ => ((num / 10 + 1) * 10) as f64,
            }
        };
        let multipler = match self {
            EnergyCutoff::Ultrafine => 1.1,
            _ => 1.0,
        };
        Ok(round_bigger_tenth(
            (cutoff_energy as f64 * multipler) as u32,
        ))
    }
}
