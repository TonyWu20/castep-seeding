use crate::param::KeywordDisplay;
use castep_seeding_derive::ParamDisplay;
use derive_builder::Builder;

mod bs_eigenvalue_tol;
mod bs_extra_bands;
mod bs_max_cg_steps;
mod bs_max_iter;
mod bs_nbands;
mod bs_re_est_k_scrn;
mod bs_xc_functional;

pub use bs_eigenvalue_tol::*;
pub use bs_extra_bands::BSExtraBands;
pub use bs_max_cg_steps::BSMaxCgSteps;
pub use bs_max_iter::BSMaxIter;
pub use bs_nbands::BSNbands;
pub use bs_re_est_k_scrn::BSReEstKScrn;
pub use bs_xc_functional::BSXcFunctional;

#[derive(Debug, Clone, Copy, ParamDisplay, PartialEq, PartialOrd, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct BandStructure {
    bs_eigenvalue_tol: Option<BSEigenvalueTol>,
    bs_max_cg_steps: Option<BSMaxCgSteps>,
    bs_max_iter: Option<BSMaxIter>,
    bs_nbands: Option<BSNbands>,
    bs_extra_bands: Option<BSExtraBands>,
    bs_re_est_k_scrn: Option<BSReEstKScrn>,
    bs_xc_functional: Option<BSXcFunctional>,
}

#[cfg(test)]
mod test {
    use super::{BSExtraBands, BandStructureBuilder};

    #[test]
    fn band_str_param() {
        let p = BandStructureBuilder::default()
            .bs_eigenvalue_tol(1e-6)
            .bs_max_iter(6000)
            .bs_extra_bands(BSExtraBands::PercExtra(72.0))
            .build()
            .unwrap();
        println!("{p}");
    }
}
