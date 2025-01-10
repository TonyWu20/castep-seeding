use crate::param::KeywordDisplay;
use castep_seeding_derive::ParamDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

mod geom_convergence_win;
mod geom_disp_tol;
mod geom_energy_tol;
mod geom_force_tol;
mod geom_frequency_est;
mod geom_max_iter;
mod geom_method;
mod geom_modulus_est;
mod geom_preconditioner;
mod geom_spin_fix;
mod geom_stress_tol;

pub use geom_convergence_win::GeomConvergenceWin;
pub use geom_disp_tol::*;
pub use geom_energy_tol::*;
pub use geom_force_tol::*;
pub use geom_frequency_est::*;
pub use geom_max_iter::GeomMaxIter;
pub use geom_method::GeomMethod;
pub use geom_modulus_est::*;
pub use geom_preconditioner::GeomPreconditioner;
pub use geom_spin_fix::GeomSpinFix;
pub use geom_stress_tol::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    Builder,
    ParamDisplay,
    Default,
)]
#[builder(setter(into, strip_option), default)]
pub struct GeomOpt {
    geom_convergence_win: Option<GeomConvergenceWin>,
    geom_disp_tol: Option<GeomDispTol>,
    geom_energy_tol: Option<GeomEnergyTol>,
    geom_force_tol: Option<GeomForceTol>,
    geom_frequency_est: Option<GeomFrequencyEst>,
    geom_max_iter: Option<GeomMaxIter>,
    geom_method: Option<GeomMethod>,
    geom_modulus_est: Option<GeomModulusEst>,
    geom_preconditioner: Option<GeomPreconditioner>,
    geom_spin_fix: Option<GeomSpinFix>,
    geom_stress_tol: Option<GeomStressTol>,
}

#[cfg(test)]
mod test {
    use super::{GeomMethod, GeomOptBuilder};

    #[test]
    fn geom_opt_test() {
        let p = GeomOptBuilder::default()
            .geom_energy_tol(5e-5)
            .geom_force_tol(0.1)
            .geom_stress_tol(0.2)
            .geom_disp_tol(0.005)
            .geom_max_iter(6000)
            .geom_method(GeomMethod::BFGS)
            .build()
            .unwrap();
        println!("{p}");
    }
}
