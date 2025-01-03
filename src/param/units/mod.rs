use super::KeywordDisplay;
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, Hash)]
pub struct Units {
    energy_unit: Option<EnergyUnit>,
    force_unit: Option<ForceUnit>,
    force_constant_unit: Option<ForceConstantUnit>,
    frequency_unit: Option<FrequencyUnit>,
    inv_length_unit: Option<InvLengthUnit>,
    length_unit: Option<LengthUnit>,
    mass_unit: Option<MassUnit>,
    pressure_unit: Option<PressureUnit>,
    time_unit: Option<TimeUnit>,
    velocity_unit: Option<VelocityUnit>,
    volume_unit: Option<VolumeUnit>,
}

impl Units {
    pub fn set_energy_unit(&mut self, energy_unit: Option<EnergyUnit>) {
        self.energy_unit = energy_unit;
    }

    pub fn set_force_unit(&mut self, force_unit: Option<ForceUnit>) {
        self.force_unit = force_unit;
    }

    pub fn set_force_constant_unit(&mut self, force_constant_unit: Option<ForceConstantUnit>) {
        self.force_constant_unit = force_constant_unit;
    }

    pub fn set_frequency_unit(&mut self, frequency_unit: Option<FrequencyUnit>) {
        self.frequency_unit = frequency_unit;
    }

    pub fn set_inv_length_unit(&mut self, inv_length_unit: Option<InvLengthUnit>) {
        self.inv_length_unit = inv_length_unit;
    }

    pub fn set_length_unit(&mut self, length_unit: Option<LengthUnit>) {
        self.length_unit = length_unit;
    }

    pub fn set_mass_unit(&mut self, mass_unit: Option<MassUnit>) {
        self.mass_unit = mass_unit;
    }

    pub fn set_pressure_unit(&mut self, pressure_unit: Option<PressureUnit>) {
        self.pressure_unit = pressure_unit;
    }

    pub fn set_time_unit(&mut self, time_unit: Option<TimeUnit>) {
        self.time_unit = time_unit;
    }

    pub fn set_velocity_unit(&mut self, velocity_unit: Option<VelocityUnit>) {
        self.velocity_unit = velocity_unit;
    }

    pub fn set_volume_unit(&mut self, volume_unit: Option<VolumeUnit>) {
        self.volume_unit = volume_unit;
    }

    pub fn energy_unit(&self) -> Option<EnergyUnit> {
        self.energy_unit
    }

    pub fn force_unit(&self) -> Option<ForceUnit> {
        self.force_unit
    }

    pub fn force_constant_unit(&self) -> Option<ForceConstantUnit> {
        self.force_constant_unit
    }

    pub fn frequency_unit(&self) -> Option<FrequencyUnit> {
        self.frequency_unit
    }

    pub fn inv_length_unit(&self) -> Option<InvLengthUnit> {
        self.inv_length_unit
    }

    pub fn length_unit(&self) -> Option<LengthUnit> {
        self.length_unit
    }

    pub fn mass_unit(&self) -> Option<MassUnit> {
        self.mass_unit
    }

    pub fn pressure_unit(&self) -> Option<PressureUnit> {
        self.pressure_unit
    }

    pub fn time_unit(&self) -> Option<TimeUnit> {
        self.time_unit
    }

    pub fn velocity_unit(&self) -> Option<VelocityUnit> {
        self.velocity_unit
    }

    pub fn volume_unit(&self) -> Option<VolumeUnit> {
        self.volume_unit
    }
}

impl Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: Vec<String> = vec![];
        if let Some(u) = self.energy_unit() {
            output.push(u.output());
        }
        if let Some(u) = self.force_unit() {
            output.push(u.output())
        }
        if let Some(u) = self.force_constant_unit() {
            output.push(u.output())
        }
        if let Some(u) = self.frequency_unit() {
            output.push(u.output())
        }
        if let Some(u) = self.inv_length_unit() {
            output.push(u.output())
        }
        if let Some(u) = self.length_unit() {
            output.push(u.output())
        }
        if let Some(u) = self.mass_unit() {
            output.push(u.output())
        }
        if let Some(u) = self.pressure_unit() {
            output.push(u.output());
        }
        if let Some(u) = self.time_unit() {
            output.push(u.output());
        }
        if let Some(u) = self.velocity_unit() {
            output.push(u.output());
        }
        if let Some(u) = self.volume_unit() {
            output.push(u.output());
        }
        if output.is_empty() {
            f.write_str("")
        } else {
            write!(f, "{}", output.join("\n"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::Units;

    #[test]
    fn units() {
        let mut unit = Units::default();
        unit.set_length_unit(Some(crate::param::units::LengthUnit::BohrA0));
        println!("output: {}", unit);
    }
}
