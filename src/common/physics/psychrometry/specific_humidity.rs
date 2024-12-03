use uom::si::ratio::{part_per_hundred};

use crate::common::physics::magnitude::Magnitude;

pub struct SpecificHumidity {
  pub magnitude: Magnitude<part_per_hundred>,
}

impl SpecificHumidity {
  pub fn new(name: String, symbol: String, value: f64, unit: part_per_hundred) -> Self {
      let magnitude = Magnitude::new(name, symbol, value, unit);
      SpecificHumidity { magnitude }
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

  pub fn unit(&self) -> &part_per_hundred {
    &self.magnitude.unit
  }

}