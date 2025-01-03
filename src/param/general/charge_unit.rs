use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub enum ChargeUnit {
    #[default]
    E,
    C,
}

impl Display for ChargeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChargeUnit::E => f.write_char('e'),
            ChargeUnit::C => f.write_char('c'),
        }
    }
}
