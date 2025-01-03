use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy)]
pub enum OptStrategy {
    Speed,
    #[default]
    Default,
    Memory,
}

impl Display for OptStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptStrategy::Speed => f.write_str("Speed"),
            OptStrategy::Default => f.write_str("Default"),
            OptStrategy::Memory => f.write_str("Memory"),
        }
    }
}
