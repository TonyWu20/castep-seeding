use crate::param::KeywordDisplay;
use std::fmt::Display;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Default,
    Builder,
)]
#[builder(setter(into, strip_option), default)]
pub struct CalculateProperties {
    stress: Option<CalculateStress>,
    densdiff: Option<CalculateDensdiff>,
    elf: Option<CalculateELF>,
    hirshfeld: Option<CalculateHirshfeld>,
}

impl CalculateProperties {
    pub fn new(
        stress: Option<CalculateStress>,
        densdiff: Option<CalculateDensdiff>,
        elf: Option<CalculateELF>,
        hirshfeld: Option<CalculateHirshfeld>,
    ) -> Self {
        Self {
            stress,
            densdiff,
            elf,
            hirshfeld,
        }
    }

    pub fn stress(&self) -> Option<CalculateStress> {
        self.stress
    }

    pub fn densdiff(&self) -> Option<CalculateDensdiff> {
        self.densdiff
    }

    pub fn elf(&self) -> Option<CalculateELF> {
        self.elf
    }

    pub fn hirshfeld(&self) -> Option<CalculateHirshfeld> {
        self.hirshfeld
    }

    pub fn set_densdiff(&mut self, densdiff: Option<CalculateDensdiff>) {
        self.densdiff = densdiff;
    }

    pub fn set_stress(&mut self, stress: Option<CalculateStress>) {
        self.stress = stress;
    }

    pub fn set_elf(&mut self, elf: Option<CalculateELF>) {
        self.elf = elf;
    }

    pub fn set_hirshfeld(&mut self, hirshfeld: Option<CalculateHirshfeld>) {
        self.hirshfeld = hirshfeld;
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct CalculateStress(bool);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct CalculateDensdiff(bool);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct CalculateELF(bool);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct CalculateHirshfeld(bool);

macro_rules! display_for_bool {
    ($($x:ident, $y: expr),+) => {
        $(
        impl Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.to_string().to_uppercase())
            }
        }
        impl KeywordDisplay for $x {
            fn field(&self) -> String {
                $y.to_string()
            }
        }
    )+


    };
}
display_for_bool!(
    CalculateStress,
    "CALCULATE_STRESS",
    CalculateELF,
    "CALCULATE_ELF",
    CalculateDensdiff,
    "CALCULATE_DENSDIFF",
    CalculateHirshfeld,
    "CALCULATE_HIRSHFELD"
);

macro_rules! from_for_bool_type {
    ($($x: ident),*) => {
        $(
    impl From<bool> for $x {
        fn from(value: bool) -> Self {
            Self(value)
        }
}
        )*
    }
}

from_for_bool_type!(
    CalculateHirshfeld,
    CalculateStress,
    CalculateDensdiff,
    CalculateELF
);

impl Display for CalculateProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = [
            self.stress().map(|b| b.output()),
            self.densdiff().map(|b| b.output()),
            self.elf().map(|b| b.output()),
            self.hirshfeld().map(|b| b.output()),
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
    use crate::param::general::calculate_props::CalculatePropertiesBuilder;

    use super::CalculateProperties;

    #[test]
    fn calc_props() {
        let p = CalculateProperties::default();
        assert!(p.to_string().is_empty());
        let p = CalculatePropertiesBuilder::default()
            .densdiff(true)
            .stress(false)
            .build()
            .unwrap();
        let target = "CALCULATE_STRESS : FALSE\nCALCULATE_DENSDIFF : TRUE";
        assert_eq!(target, p.to_string());
    }
}
