use std::{fmt::Display, thread::Builder};

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

mod basis_de_dloge;
mod basis_precision;
mod cut_off_energy;
mod fine_gmax;
mod fine_grid_scale;
mod finite_basis_corr;
mod finite_basis_npoints;
mod finite_basis_spacing;
mod fixed_npw;
mod grid_scale;

pub use basis_de_dloge::BasisDeDloge;
pub use basis_precision::BasisPrecision;
pub use cut_off_energy::CutoffEnergy;
pub use fine_gmax::FineGMax;
pub use fine_grid_scale::FineGridScale;
pub use finite_basis_corr::FiniteBasisCorr;
pub use finite_basis_npoints::FiniteBasisNPoints;
pub use finite_basis_spacing::{
    FiniteBasisSpacing, FiniteBasisSpacingBuilder, FiniteBasisSpacingBuilderError,
};
pub use fixed_npw::FixedNPW;
pub use grid_scale::GridScale;

use super::KeywordDisplay;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct BasisSet {
    basis_de_dloge: Option<BasisDeDloge>,
    basis_precision: Option<BasisPrecision>,
    cut_off_energy: Option<CutoffEnergy>,
    fine_gmax: Option<FineGMax>,
    fine_grid_scale: Option<FineGridScale>,
    finite_basis_corr: Option<FiniteBasisCorr>,
    finite_basis_npoints: Option<FiniteBasisNPoints>,
    finite_basis_spacing: Option<FiniteBasisSpacing>,
    fixed_npw: Option<FixedNPW>,
    grid_scale: Option<GridScale>,
}

impl BasisSetBuilder {
    fn validate(builder: &BasisSetBuilder) -> Result<(), String> {
        // 1. basis_de_dloge required if finite_basis_corr=Manual (1)
        if let Some(Some(FiniteBasisCorr::Manual)) = builder.finite_basis_corr {
            match builder.basis_de_dloge {
                Some(Some(_)) => (),
                _ => return Err("BASIS_DE_DLOGE is required when FINITE_BASIS_CORR=1".to_string()),
            }
        }
        // 2. Basis_precision and cut_off_energy is mutually exclusive. Only one of them is permitted
        if let Some(Some(_)) = builder.basis_precision {
            if let Some(Some(_)) = builder.cut_off_energy {
                return Err(
                    "BASIS_PRECISION and CUT_OFF_ENERGY cannot be both specified".to_string(),
                );
            }
        } else if let Some(Some(_)) = builder.cut_off_energy {
            if let Some(Some(_)) = builder.basis_precision {
                return Err(
                    "BASIS_PRECISION and CUT_OFF_ENERGY cannot be both specified".to_string(),
                );
            }
        }
        // 3. If use `FixedNPW`, `FiniteBasisCorr` must be 0 (NoCorrection)
        if let Some(Some(_)) = builder.fixed_npw {
            match builder.finite_basis_corr {
                Some(Some(FiniteBasisCorr::NoCorrection)) => (),
                _ => {
                    return Err(
                        "If use `FixedNPW`, `FiniteBasisCorr` must be 0 (NoCorrection)".to_string(),
                    )
                }
            }
        }
        Ok(())
    }
}

impl Display for BasisSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = [
            self.basis_de_dloge.map(|v| v.output()),
            self.basis_precision.map(|v| v.output()),
            self.cut_off_energy.map(|v| v.output()),
            self.fine_gmax.map(|v| v.output()),
            self.fine_grid_scale.map(|v| v.output()),
            self.finite_basis_corr.map(|v| v.output()),
            self.finite_basis_npoints.map(|v| v.output()),
            self.finite_basis_spacing.map(|v| v.output()),
            self.fixed_npw.map(|v| v.output()),
            self.grid_scale.map(|v| v.output()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("\n");
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod test {
    use crate::param::basis_set::{BasisPrecision, FiniteBasisCorr};

    use super::BasisSetBuilder;

    #[test]
    fn basis_set() {
        let b = BasisSetBuilder::default().build();
        assert!(b.is_ok());
        let b = BasisSetBuilder::default().fixed_npw(true).build();
        assert!(b.is_err());
        let b = BasisSetBuilder::default()
            .fixed_npw(true)
            .finite_basis_corr(FiniteBasisCorr::NoCorrection)
            .build();
        assert!(b.is_ok());
        println!("{}", b.unwrap());
        let b = BasisSetBuilder::default()
            .cut_off_energy(340.0)
            .fine_grid_scale(1.5)
            .build();
        assert!(b.is_ok());
        println!("{}", b.unwrap());
        let b = BasisSetBuilder::default()
            .basis_precision(BasisPrecision::Coarse)
            .build();
        assert!(b.is_ok());
        let b = BasisSetBuilder::default()
            .basis_precision(BasisPrecision::Coarse)
            .cut_off_energy(320.0)
            .build();
        assert!(b.is_err());
        dbg!(b.err());
        let b = BasisSetBuilder::default().basis_de_dloge(-1.2345).build();
        assert!(b.is_ok());
        println!("{}", b.unwrap());
        let b = BasisSetBuilder::default()
            .finite_basis_corr(FiniteBasisCorr::Manual)
            .build();
        assert!(b.is_err());
        dbg!(b.err());
        let b = BasisSetBuilder::default()
            .finite_basis_corr(FiniteBasisCorr::Manual)
            .basis_de_dloge(2.0)
            .build();
        assert!(b.is_ok());
        println!("{}", b.unwrap());
    }
}
