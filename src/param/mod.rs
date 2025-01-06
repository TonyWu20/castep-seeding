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
pub use general::*;
use geom_opt::GeomOpt;
use population::Population;
use units::Units;
use xc_correlation::XcParam;

#[derive(Debug, Clone)]
pub struct CastepParam {
    general_keywords: Option<General>,
    units: Option<Units>,
    band_structure: Option<BandStructure>,
    basis_set: Option<BasisSet>,
    electro_min: Option<ElectroMinimization>,
    electronic: Option<Electronic>,
    geom_opt: Option<GeomOpt>,
    xc_correlation: Option<XcParam>,
    population: Option<Population>,
}

pub trait KeywordDisplay: Display {
    fn field(&self) -> String;
    fn output(&self) -> String {
        format!("{} : {}", self.field(), self)
    }
}
