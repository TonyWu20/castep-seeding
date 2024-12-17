use std::fmt::{Display, Write};

#[derive(Debug, Clone, Default)]
pub struct GeneralKeywords {
    backup_interval: i64,                // 0
    calculate_stress: bool,              // false
    calculate_densdiff: bool,            // false
    calculate_elf: bool,                 // false
    calculate_hirshfeld: bool,           // false
    charge_unit: ChargeUnit,             // E
    checkpoint: String,                  // seedname.check
    comment: String,                     // Blank
    continuation: Option<bool>,          // Null
    data_distribution: DataDistribution, // Default
    num_backup_iter: i64,                // 5
    opt_strategy: OptStrategy,           // Default
    page_wvfns: i32,                     // 0
    print_clock: bool,                   // true
    print_memory_usage: bool,            // true
    rand_seed: i32,                      // 0
    reuse: Option<bool>,                 // NULL
    run_time: i32,                       // 0
    stop: bool,                          // Example: STOP
    task: CastepTask,
    write_checkpoint: WriteCheckpoint,
    write_formatted_density: bool,   // false
    write_formatted_elf: bool,       // false
    write_formatted_potential: bool, // false
    write_orbitals: bool,            // false unless `task` is `BANDSTRUCTURE`
}

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

#[derive(Debug, Clone, Copy, Default)]
pub enum ChargeUnit {
    #[default]
    E,
    C,
}

impl Display for ChargeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChargeUnit::E => f.write_char('e'),
            ChargeUnit::C => f.write_char('c'),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum DataDistribution {
    Kpoint,
    Gvector,
    Mixed,
    #[default]
    Default,
}

impl Display for DataDistribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataDistribution::Kpoint => f.write_str("kpoint"),
            DataDistribution::Gvector => f.write_str("gvector"),
            DataDistribution::Mixed => f.write_str("mixed"),
            DataDistribution::Default => f.write_str("Default"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum OptStrategy {
    Speed,
    #[default]
    Default,
    Memory,
}

impl Display for OptStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptStrategy::Speed => f.write_str("Speed"),
            OptStrategy::Default => f.write_str("Default"),
            OptStrategy::Memory => f.write_str("Memory"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WriteCheckpoint {
    values: WriteCheckpointValue,
    options: WriteCheckpointOption,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum WriteCheckpointValue {
    None,
    Minimal,
    Both,
    #[default]
    All,
    Full,
}

impl Display for WriteCheckpointValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteCheckpointValue::None => f.write_str("NONE"),
            WriteCheckpointValue::Minimal => f.write_str("Minimal"),
            WriteCheckpointValue::Both => f.write_str("Both"),
            WriteCheckpointValue::All => f.write_str("ALL"),
            WriteCheckpointValue::Full => f.write_str("Full"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WriteCheckpointOption {
    Success(WriteCheckpointValue),
    Failure(WriteCheckpointValue),
    Backup(WriteCheckpointValue),
}

impl Display for WriteCheckpointOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteCheckpointOption::Success(v) => write!(f, "SUCCESS={v}"),
            WriteCheckpointOption::Failure(v) => write!(f, "FAILURE={v}"),
            WriteCheckpointOption::Backup(v) => write!(f, "BACKUP={v}"),
        }
    }
}

impl Default for WriteCheckpointOption {
    fn default() -> Self {
        Self::Success(WriteCheckpointValue::All)
    }
}
