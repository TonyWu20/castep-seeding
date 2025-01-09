mod efermi_tol;
mod elec_convergence_win;
mod elec_dump_file;
mod elec_eigenvalue_tol;
mod elec_energy_tol;
mod elec_restore_file;
mod electronic_minimizer;
mod fix_occupancy;
mod max_cg_steps;
mod max_scf_cycles;
mod max_sd_steps;
mod metals_method;
mod num_dump_cycles;
mod smearing_scheme;
mod smearing_width;
mod spin_fix;

use crate::param::KeywordDisplay;
use castep_seeding_derive::ParamDisplay;

use derive_builder::Builder;
pub use efermi_tol::*;
pub use elec_convergence_win::ElecConvergenceWin;
pub use elec_dump_file::ElecDumpFile;
pub use elec_eigenvalue_tol::*;
pub use elec_energy_tol::*;
pub use elec_restore_file::ElecRestoreFile;
pub use electronic_minimizer::ElectronicMinimizer;
pub use fix_occupancy::FixOccupancy;
pub use max_cg_steps::MaxCgSteps;
pub use max_scf_cycles::MaxScfCycles;
pub use max_sd_steps::MaxSdSteps;
pub use metals_method::MetalsMethod;
pub use num_dump_cycles::NumDumpCycles;
pub use smearing_scheme::SmearingScheme;
pub use smearing_width::SmearingWidth;
pub use spin_fix::SpinFix;

#[derive(Debug, Clone, ParamDisplay, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct ElectroMinimization {
    efermi_tol: Option<EFermiTol>,
    elec_convergence_win: Option<ElecConvergenceWin>,
    #[param_display(use_ref = true)]
    elec_dump_file: Option<ElecDumpFile>,
    elec_eigenvalue_tol: Option<ElecEigenvalueTol>,
    elec_energy_tol: Option<ElecEnergyTol>,
    #[param_display(use_ref = true)]
    elec_restore_file: Option<ElecRestoreFile>,
    electronic_minimizer: Option<ElectronicMinimizer>,
    fix_occupancy: Option<FixOccupancy>,
    max_cg_steps: Option<MaxCgSteps>,
    max_scf_cycles: Option<MaxScfCycles>,
    max_sd_steps: Option<MaxSdSteps>,
    metals_method: Option<MetalsMethod>,
    num_dump_cycles: Option<NumDumpCycles>,
    smearing_scheme: Option<SmearingScheme>,
    smearing_width: Option<SmearingWidth>,
    spin_fix: Option<SpinFix>,
}

#[cfg(test)]
mod test {
    use crate::param::electro_min::MetalsMethod;

    use super::ElectroMinimizationBuilder;

    #[test]
    fn electro_min() {
        let p = ElectroMinimizationBuilder::default()
            .elec_energy_tol(1e-6)
            .elec_eigenvalue_tol(1e-6)
            .metals_method(MetalsMethod::DM)
            .max_scf_cycles(6000)
            .build()
            .unwrap();
        println!("{p}")
    }
}
