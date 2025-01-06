use super::KeywordDisplay;
use derive_builder::Builder;
use std::fmt::Display;

pub use energy_unit::EnergyUnit;
pub use force_constant_unit::ForceConstantUnit;
pub use force_unit::ForceUnit;
pub use frequency_unit::FrequencyUnit;
pub use inv_length_unit::InvLengthUnit;
pub use length_unit::LengthUnit;
pub use mass_unit::MassUnit;
pub use pressure_unit::PressureUnit;
pub use serde::{Deserialize, Serialize};
pub use time_unit::TimeUnit;
pub use velocity_unit::VelocityUnit;
pub use volume_unit::VolumeUnit;

mod energy_unit;
mod force_constant_unit;
mod force_unit;
mod frequency_unit;
mod inv_length_unit;
mod length_unit;
mod mass_unit;
mod pressure_unit;
mod time_unit;
mod velocity_unit;
mod volume_unit;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Units {
    pub energy_unit: Option<EnergyUnit>,
    pub force_unit: Option<ForceUnit>,
    pub force_constant_unit: Option<ForceConstantUnit>,
    pub frequency_unit: Option<FrequencyUnit>,
    pub inv_length_unit: Option<InvLengthUnit>,
    pub length_unit: Option<LengthUnit>,
    pub mass_unit: Option<MassUnit>,
    pub pressure_unit: Option<PressureUnit>,
    pub time_unit: Option<TimeUnit>,
    pub velocity_unit: Option<VelocityUnit>,
    pub volume_unit: Option<VolumeUnit>,
}

impl Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = [
            self.energy_unit.map(|v| v.output()),
            self.force_unit.map(|v| v.output()),
            self.force_constant_unit.map(|v| v.output()),
            self.frequency_unit.map(|v| v.output()),
            self.inv_length_unit.map(|v| v.output()),
            self.length_unit.map(|v| v.output()),
            self.mass_unit.map(|v| v.output()),
            self.pressure_unit.map(|v| v.output()),
            self.time_unit.map(|v| v.output()),
            self.velocity_unit.map(|v| v.output()),
            self.volume_unit.map(|v| v.output()),
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
    use crate::param::units::{LengthUnit, UnitsBuilder};

    #[test]
    fn units() {
        let unit = UnitsBuilder::default()
            .length_unit(LengthUnit::Nanometer)
            .build()
            .unwrap();
        println!("output: {}", unit);
    }
}
