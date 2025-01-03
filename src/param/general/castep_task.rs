#[derive(Debug, Clone, Copy, Default)]
pub enum CastepTask {
    BandStructure, // calculates band structure properties.
    #[default]
    GeometryOptimization, // searches for a minimum energy structure.
                   //  TODO: Future
                   // SinglePoint,            // performs a single-point energy calculation.
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
