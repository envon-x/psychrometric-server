
// use uom::si::mass_per_energy::kilogram_per_kilowatt_hour;

use uom::si::ratio::part_per_thousand;

use crate::common::physics::magnitude::Magnitude;

pub struct SaturatedHumidityRatio {
  // pub magnitude: Magnitude<kilogram_per_kilogram>,
  pub magnitude: Magnitude<part_per_thousand>,
}

impl SaturatedHumidityRatio {
  pub fn new(name: String, symbol: String, value: f64, unit: part_per_thousand) -> Self {
      let magnitude = Magnitude::new(name, symbol, value, unit);
      SaturatedHumidityRatio { magnitude }
  }

  pub fn name(&self) -> &str {
    &self.magnitude.name
  }

  pub fn symbol(&self) -> &str {
    &self.magnitude.symbol
  }

  pub fn value(&self) -> f64 {
      self.magnitude.value
  }

  pub fn unit(&self) -> &part_per_thousand {
    &self.magnitude.unit
  }

}