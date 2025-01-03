#![allow(dead_code)]
mod general;
mod units;
mod band_structure {
    #[derive(Debug, Clone)]
    pub struct BandStructure;
}
mod basis_set {
    #[derive(Debug, Clone)]
    pub struct BasisSet;
}

mod electro_min {
    #[derive(Debug, Clone)]
    pub struct ElectroMinimization;
}

mod electronic {
    #[derive(Debug, Clone)]
    pub struct Electronic;
}

mod geom_opt {
    #[derive(Debug, Clone)]
    pub struct GeomOpt;
}

mod xc_correlation {
    #[derive(Debug, Clone)]
    pub struct XcParam;
}

mod population {
    #[derive(Debug, Clone)]
    pub struct Population;
}

use std::fmt::Display;

use band_structure::BandStructure;
use basis_set::BasisSet;
use electro_min::ElectroMinimization;
use electronic::Electronic;
use general::General;
use geom_opt::GeomOpt;
use population::Population;
use xc_correlation::XcParam;

#[derive(Debug, Clone)]
pub struct CastepParam {
    general_keywords: General,
    band_structure: BandStructure,
    basis_set: BasisSet,
    electro_min: ElectroMinimization,
    electronic: Electronic,
    geom_opt: GeomOpt,
    xc_correlation: XcParam,
    population: Population,
}

pub trait KeywordDisplay: Display {
    fn field(&self) -> String;
    fn output(&self) -> String {
        format!("{} : {}", self.field(), self)
    }
}
