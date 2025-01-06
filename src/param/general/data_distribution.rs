use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;
#[derive(
    Debug, Default, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
/// This keyword determines the parallelization strategy used. Available options are:
/// `Kpoint` - only k-point parallelization will be used (best scalability)
/// `Gvector` - only g-vector parallelization will be used (worst scalability)
/// `Mixed` - a combination of k-point and g-vector parallelization will be used
/// `Default` - the optimal parallelization strategy for the architecture will be used
/// The Default setting is appropriate in most situations.
/// The Kpoint option is available only when the number of processors is a factor of the number of k-points.
/// The Gvector option is appropriate for calculations involving large systems where often only one k-point is used.
/// The Mixed option is available when the number of processors and the number of k-points have a common factor (for example 6 k-points and 4 processors).
/// The differences in the scaling properties of the different schemes dictate the number of processors that should be used. For example, a calculation that uses 16 k-points is likely to finish faster on 4 processors (using Kpoint distribution) than on 6 processors (using Mixed distribution) and most likely faster than on 9 processors (using Gvector distribution).
/// # Default
/// `Default`
/// # Example
/// `DATA_DISTRIBUTION : Gvector`
pub enum DataDistribution {
    Kpoint,
    Gvector,
    Mixed,
    #[default]
    Default,
}

impl Display for DataDistribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataDistribution::Kpoint => f.write_str("Kpoint"),
            DataDistribution::Gvector => f.write_str("Gvector"),
            DataDistribution::Mixed => f.write_str("Mixed"),
            DataDistribution::Default => f.write_str("Default"),
        }
    }
}

impl KeywordDisplay for DataDistribution {
    fn field(&self) -> String {
        "DATA_DISTRIBUTION".to_string()
    }
}
