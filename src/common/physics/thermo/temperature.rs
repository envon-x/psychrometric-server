use crate::common::physics::magnitude::Magnitude;

extern crate uom;
use uom::si::thermodynamic_temperature::kelvin;

pub trait Temperature {
  fn magnitude(&self) -> &Magnitude<kelvin>;
}


pub struct TemperatureThermodynamic {
  pub magnitude: Magnitude<kelvin>,
}

impl Temperature for TemperatureThermodynamic {
  fn magnitude(&self) -> &Magnitude<kelvin> {
      &self.magnitude
  }
}


pub struct TemperatureDryBulb {
  pub magnitude: Magnitude<kelvin>,
}

impl Temperature for TemperatureDryBulb {
  fn magnitude(&self) -> &Magnitude<kelvin> {
      &self.magnitude
  }
}



pub struct StandardAtmTemperature {
  pub magnitude: Magnitude<kelvin>,
}

impl Temperature for StandardAtmTemperature {
  fn magnitude(&self) -> &Magnitude<kelvin> {
      &self.magnitude
  }
}



pub struct TemperatureDewPoint {
  pub magnitude: Magnitude<kelvin>,
}

impl Temperature for TemperatureDewPoint {
  fn magnitude(&self) -> &Magnitude<kelvin> {
      &self.magnitude
  }
}



pub struct TemperatureWetBulb {
  pub magnitude: Magnitude<kelvin>,
}

impl Temperature for TemperatureWetBulb {
  fn magnitude(&self) -> &Magnitude<kelvin> {
      &self.magnitude
  }
}