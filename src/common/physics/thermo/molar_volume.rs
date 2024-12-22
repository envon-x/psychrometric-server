use crate::common::{physics::magnitude::Magnitude, utils::error::NumberError};

extern crate uom;
use uom::si::molar_volume::cubic_meter_per_mole;

#[derive(Debug)]
pub struct Volume<Unit> {
  magnitude: Magnitude<Unit>,
}

impl<Unit> Volume<Unit> {
  pub fn new(name: String, symbol: String, value: f64, unit: Unit) -> Self {
    // if value <= 0.0 {
    //   return Err(NumberError::NegativeNumber);
    // }

    let volume = Volume {
      magnitude: Magnitude::new(name, symbol, value, unit),
    };

    volume
  }
  
  pub fn magnitude(&self) -> &Magnitude<Unit> {
    &self.magnitude
  }
}

// Specific Types of MolarVolume
pub type MolarVolume = Volume<cubic_meter_per_mole>;
pub type DryGasVolume = Volume<cubic_meter_per_mole>;
pub type MoistureGasVolume = Volume<cubic_meter_per_mole>;
