
use uom::si::molar_energy::joule_per_mole;

use crate::common::physics::magnitude::Magnitude;

pub struct HumidityRatio {
  pub magnitude: Magnitude<joule_per_mole>,
}

impl HumidityRatio {
  pub fn new(name: String, symbol: String, value: f64) -> Self {
      let magnitude = Magnitude::new(name, symbol, value, joule_per_mole);
      HumidityRatio { magnitude }
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

  pub fn unit(&self) -> &joule_per_mole {
    &self.magnitude.unit
  }

}