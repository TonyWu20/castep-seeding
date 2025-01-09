use crate::param::KeywordDisplay;
use castep_seeding_derive::ParamDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

mod mix_charge_amp {
    use castep_seeding_derive::KeywordDisplay;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplay)]
    #[keyword_display(field="MIX_CHARGE_AMP", from=f64, value=f64)]
    /// This keyword determines the mixing amplitude for the charge density in the density mixing procedure.
    /// # Default
    /// 0.8
    /// # Example
    /// MIX_CHARGE_AMP : 0.5
    pub struct MixChargeAmp(f64);
}

use mix_charge_amp::MixChargeAmp;

#[derive(Debug, Clone, Copy, Builder, Default, ParamDisplay, Serialize, Deserialize)]
#[builder(setter(into, strip_option), default)]
pub struct DensityMixingParams {
    mix_charge_amp: Option<MixChargeAmp>,
    mix_charge_gmax: Option<MixChargeGmax>,
    mix_cut_off_energy: Option<MixCutOffEnergy>,
    mix_history_length: Option<MixHistoryLength>,
    mix_metric_q: Option<MixMetricQ>,
    mix_spin_amp: Option<MixSpinAmp>,
    mix_spin_gmax: Option<MixSpinGmax>,
    mixing_scheme: Option<MixingScheme>,
}
