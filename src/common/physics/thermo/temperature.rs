use crate::common::physics::magnitude::Magnitude;

extern crate uom;
use uom::si::thermodynamic_temperature::kelvin;

pub struct Temperature<Unit> {
    magnitude: Magnitude<Unit>,
}

impl<Unit> Temperature<Unit> {
    pub fn new(name: String, symbol: String, value: f64, unit: Unit) -> Self {
        Temperature {
            magnitude: Magnitude::new(name, symbol, value, unit),
        }
    }

    pub fn magnitude(&self) -> &Magnitude<Unit> {
        &self.magnitude
    }
}

// Specific Types of Temperature
pub type TemperatureDryBulb = Temperature<kelvin>;
pub type StandardAtmTemperature = Temperature<kelvin>;
pub type TemperatureDewPoint = Temperature<kelvin>;
pub type TemperatureWetBulb = Temperature<kelvin>;

pub type ThermodynamicTemperature = Temperature<kelvin>;
pub type VaporPressureDeficit = Temperature<kelvin>;
pub type SeaLevelPressure = Temperature<kelvin>;
