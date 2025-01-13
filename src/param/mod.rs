#![allow(dead_code)]
mod band_structure;
mod basis_set;
mod density_mixing_params;
mod electro_min;
mod electronic;
mod general;
mod geom_opt;
mod population_analysis;
mod pseudopotentials;
mod units;
mod xc_correlation;

use std::fmt::Display;

use band_structure::BandStructure;
pub use basis_set::*;
use castep_seeding_derive::ParamDisplay;
pub use density_mixing_params::*;
pub use electro_min::*;
pub use electronic::*;
pub use general::*;
pub use geom_opt::*;
pub use population_analysis::*;
pub use pseudopotentials::*;
pub use units::*;
pub use xc_correlation::*;

#[derive(Debug, Clone, ParamDisplay)]
#[param_display(use_display = true)]
pub struct CastepParam {
    #[param_display(use_ref = true)]
    general_keywords: Option<General>,
    units: Option<Units>,
    pseudopotentials: Option<Pseudopotentials>,
    band_structure: Option<BandStructure>,
    basis_set: Option<BasisSet>,
    electronic: Option<ElectronicParam>,
    #[param_display(use_ref = true)]
    electro_min: Option<ElectroMinimization>,
    density_mixing: Option<DensityMixingParams>,
    population_analysis: Option<PopulationAnalysis>,
    geom_opt: Option<GeomOpt>,
    #[param_display(use_ref = true)]
    xc_correlation: Option<XcParam>,
}

pub trait KeywordDisplay: Display {
    fn field(&self) -> String;
    fn output(&self) -> String {
        format!("{} : {}", self.field(), self)
    }
}
