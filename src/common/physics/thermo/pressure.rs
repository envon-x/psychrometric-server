use crate::common::physics::magnitude::Magnitude;

use uom::si::pressure::pascal;

pub struct Pressure<Unit> {
    magnitude: Magnitude<Unit>,
}

impl<Unit> Pressure<Unit> {
    pub fn new(name: String, symbol: String, value: f64, unit: Unit) -> Self {
        Pressure {
            magnitude: Magnitude::new(name, symbol, value, unit),
        }
    }

    pub fn magnitude(&self) -> &Magnitude<Unit> {
        &self.magnitude
    }
}

// Specific Types of Pressure
pub type AbsolutePressure = Pressure<pascal>;
pub type AtmosphericPressure = Pressure<pascal>;
pub type VaporPressure = Pressure<pascal>;
pub type StandardAtmPressure = Pressure<pascal>;
pub type SaturatedVaporPressure = Pressure<pascal>;
pub type VaporPressureDeficit = Pressure<pascal>;
pub type SeaLevelPressure = Pressure<pascal>;
pub type StationPressure = Pressure<pascal>;