use std::fmt::Display;
#[derive(Debug, Default, Clone, Copy)]
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
            DataDistribution::Kpoint => f.write_str("kpoint"),
            DataDistribution::Gvector => f.write_str("gvector"),
            DataDistribution::Mixed => f.write_str("mixed"),
            DataDistribution::Default => f.write_str("Default"),
        }
    }
}
