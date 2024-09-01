// pub struct AbsoluteHumidity<Unit> {
//   pub name: String,
//   pub value: f64,
//   pub unit: Unit
// }

// impl<Unit> AbsoluteHumidity<Unit> {
//   pub fn new(value: f64, unit: Unit) -> Self {
//       AbsoluteHumidity {name: String::from("absolute_humidity"), value, unit }
//   }

//   pub fn value(&self) -> f64 {
//       self.value
//   }
// }


use uom::si::ratio::part_per_thousand;

use crate::common::physics::magnitude::Magnitude;
pub struct AbsoluteHumidity {
  pub magnitude: Magnitude<part_per_thousand>,
}

impl AbsoluteHumidity {
  pub fn new(name: String, symbol: String, value: f64, unit: Option<part_per_thousand>) -> Self {
      let magnitude = Magnitude::new(name, symbol, value, unit.unwrap_or(part_per_thousand));
      AbsoluteHumidity { magnitude }
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


  // Mutable access.
  fn name_mut(&mut self) -> &mut String {
    &mut self.magnitude.name
  }

  fn value_mut(&mut self) -> &mut f64 {
    &mut self.magnitude.value
  }

}