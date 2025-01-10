#![allow(dead_code)]
mod general;
mod pseudopotentials;
mod units;
mod band_structure {
    #[derive(Debug, Clone)]
    pub struct BandStructure;
}
mod basis_set;
mod density_mixing_params;
mod electro_min;

mod electronic;

mod geom_opt {
    #[derive(Debug, Clone)]
    pub struct GeomOpt;
}

mod population_analysis;
mod xc_correlation;

use std::fmt::Display;

use band_structure::BandStructure;
pub use basis_set::*;
pub use density_mixing_params::*;
pub use electro_min::*;
pub use electronic::*;
pub use general::*;
use geom_opt::GeomOpt;
pub use population_analysis::*;
pub use pseudopotentials::*;
pub use units::*;
use xc_correlation::XcParam;

#[derive(Debug, Clone)]
pub struct CastepParam {
    general_keywords: Option<General>,
    units: Option<Units>,
    pseudopotentials: Option<Pseudopotentials>,
    band_structure: Option<BandStructure>,
    basis_set: Option<BasisSet>,
    electronic: Option<ElectronicParam>,
    electro_min: Option<ElectroMinimization>,
    density_mixing: Option<DensityMixingParams>,
    population_analysis: Option<PopulationAnalysis>,
    geom_opt: Option<GeomOpt>,
    xc_correlation: Option<XcParam>,
}

pub trait KeywordDisplay: Display {
    fn field(&self) -> String;
    fn output(&self) -> String {
        format!("{} : {}", self.field(), self)
    }
}
