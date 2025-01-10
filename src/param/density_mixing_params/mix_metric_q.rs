use castep_seeding_derive::KeywordDisplayStruct;
use serde::{Deserialize, Serialize};

use crate::param::InvLengthUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplayStruct,
)]
#[keyword_display(field = "MIX_METRIC_Q", display_format = "{} {}")]
/// This keyword determines the weighting factor for the densities used in
/// the density mixing scheme.
/// CASTEP uses a weighting factor when evaluating scalar products of densities.
/// The factor depends on the wave vector q, according to:
/// `f(q) = (q2 + q12)/q2`
/// where q1 is the value of the MIX_METRIC_Q parameter.
/// CASTEP sets the value of q1 automatically if MIX_METRIC_Q is not specified.
/// # Default
/// -1 - CASTEP will automatically select the appropriate value
/// # Example
/// `MIX_METRIC_Q : 20.0 1/ang`
pub struct MixMetricQ {
    pub q: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<InvLengthUnit>,
}

impl Default for MixMetricQ {
    fn default() -> Self {
        Self {
            q: -1.0,
            unit: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::MixMetricQ;

    #[test]
    fn mix_metric_q() {
        let q = MixMetricQ::default();
        println!("{}", q.output());
    }
}
