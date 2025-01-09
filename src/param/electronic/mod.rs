use castep_seeding_derive::ParamDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub use bands_option::*;
pub use charge::Charge;
pub use nelectrons::{NDown, NElectrons, NUp};
pub use spin::Spin;
pub use spin_polarised::SpinPolarised;

use crate::param::KeywordDisplay;

mod bands_option;
mod charge;
mod nelectrons;
mod spin;
mod spin_polarised;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, Builder, ParamDisplay)]
#[non_exhaustive]
#[builder(setter(into, strip_option), default)]
/// `Electronic parameters` section of castep `.param`
pub struct ElectronicParam {
    charge: Option<Charge>,
    #[param_display(display=to_string())]
    bands_option: Option<BandsOption>,
    spin_polarised: Option<SpinPolarised>,
    nelectrons: Option<NElectrons>,
    nup: Option<NUp>,
    ndown: Option<NDown>,
    spin: Option<Spin>, // SEDC_APPLY
                        // SEDC_D_G06
                        // SEDC_D_JCHS
                        // SEDC_D_TS
                        // SEDC_LAMBDA_OBS
                        // SEDC_N_OBS
                        // SEDC_S6_G06
                        // SEDC_S6_JCHS
                        // SEDC_SR_JCHS
                        // SEDC_SR_TS
                        // SEDC_SCHEME
}

#[cfg(test)]
mod test {
    use super::{
        bands_option::{BandsOptionBuilder, ExtraBands},
        ElectronicParamBuilder,
    };

    #[test]
    fn electronic_param() {
        let elec_param = ElectronicParamBuilder::default()
            .spin_polarised(true)
            .spin(2.0)
            .bands_option(
                BandsOptionBuilder::default()
                    .extra_bands(ExtraBands::PercExtraBands(72.0))
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        println!("{}", elec_param);
    }
}
