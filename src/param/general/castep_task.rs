use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[non_exhaustive]
pub enum CastepTask {
    BandStructure,        // calculates band structure properties.
    GeometryOptimization, // searches for a minimum energy structure.
    #[default]
    SinglePoint, // performs a single-point energy calculation.
                          //  TODO: Future
                          // MolecularDynamics,      // performs a molecular dynamics calculation.
                          // Optics,                 // calculates optical properties.
                          // Phonon, // performs a linear response calculation to determine phonon frequencies and eigenvectors.
                          // Efield, // performs an electric field linear response calculation to determine dielectric permittivity and polarizability.
                          // PhononEfield, // performs a linear response calculation to determine phonon frequencies and eigenvectors, and an electric field linear response calculation to determine dielectric permittivity and polarizability.
                          // TransitionStateSearch, // performs a transition-state search.
                          // MagRes,       // performs an NMR calculation.
                          // Elnes,        // performs core level spectroscopy calculation.
                          // ElectronicSpectroscopy, // performs electronic spectroscopy calculation.
                          // Autosolvation, // performs a free energy of solvation calculation.
}

impl Display for CastepTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CastepTask::BandStructure => f.write_str("bandstructure"),
            CastepTask::GeometryOptimization => f.write_str("GeometryOptimization"),
            CastepTask::SinglePoint => f.write_str("SinglePoint"),
        }
    }
}

impl KeywordDisplay for CastepTask {
    fn field(&self) -> String {
        "TASK".to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::CastepTask;

    #[test]
    fn castep_task() {
        let task = CastepTask::default();
        assert_eq!("TASK : SinglePoint", task.output());
        let bandstr = CastepTask::BandStructure;
        assert_eq!("TASK : bandstructure", bandstr.output());
        let geom_opt = CastepTask::GeometryOptimization;
        assert_eq!("TASK : GeometryOptimization", geom_opt.output());
    }
}
