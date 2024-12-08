use crate::common::physics::magnitude::Magnitude;

extern crate uom;
use uom::si::molar_volume::cubic_meter_per_mole;

#[derive(Debug)]
pub struct Volume<Unit> {
    magnitude: Magnitude<Unit>,
}

impl<Unit> Volume<Unit> {
    pub fn new(name: String, symbol: String, value: f64, unit: Unit) -> Self {
        Volume {
            magnitude: Magnitude::new(name, symbol, value, unit),
        }
    }

    pub fn magnitude(&self) -> &Magnitude<Unit> {
        &self.magnitude
    }
}

// Specific Types of MolarVolume
pub type MolarVolume = Volume<cubic_meter_per_mole>;
pub type DryGasVolume = Volume<cubic_meter_per_mole>;
pub type MoistureGasVolume = Volume<cubic_meter_per_mole>;
