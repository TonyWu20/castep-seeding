use crate::param::KeywordDisplay;
use castep_seeding_derive::ParamDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

mod mix_charge_amp;
mod mix_charge_gmax;
mod mix_cut_off_energy;
mod mix_history_length;
mod mix_metric_q;
mod mix_spin_amp;
mod mix_spin_gmax;
mod mixing_scheme;

pub use mix_charge_amp::MixChargeAmp;
pub use mix_charge_gmax::MixChargeGmax;
pub use mix_cut_off_energy::MixCutOffEnergy;
pub use mix_history_length::MixHistoryLength;
pub use mix_metric_q::MixMetricQ;
pub use mix_spin_amp::MixSpinAmp;
pub use mix_spin_gmax::MixSpinGmax;
pub use mixing_scheme::MixingScheme;

#[derive(Debug, Clone, Copy, Builder, Default, ParamDisplay, Serialize, Deserialize)]
#[builder(setter(into, strip_option), default)]
pub struct DensityMixingParams {
    mixing_scheme: Option<MixingScheme>,
    mix_charge_amp: Option<MixChargeAmp>,
    mix_spin_amp: Option<MixSpinAmp>,
    mix_charge_gmax: Option<MixChargeGmax>,
    mix_spin_gmax: Option<MixSpinGmax>,
    mix_cut_off_energy: Option<MixCutOffEnergy>,
    mix_metric_q: Option<MixMetricQ>,
    mix_history_length: Option<MixHistoryLength>,
}

#[cfg(test)]
mod test {
    use super::{DensityMixingParamsBuilder, MixChargeGmax, MixSpinAmp, MixSpinGmax, MixingScheme};

    #[test]
    fn dm_param() {
        let p = DensityMixingParamsBuilder::default()
            .mix_spin_amp(2.0)
            .mix_spin_gmax(MixSpinGmax::default())
            .mix_charge_amp(0.5)
            .mix_charge_gmax(MixChargeGmax {
                gmax: 1.5,
                unit: None,
            })
            .mix_history_length(20)
            .mixing_scheme(MixingScheme::Pulay)
            .build()
            .unwrap();
        println!("{p}");
    }
}
