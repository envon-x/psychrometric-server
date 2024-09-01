extern crate uom;
use uom::si::molar_energy::joule_per_mole;


use crate::common::physics::magnitude::Magnitude;

pub struct MolarEnthalpy {
  pub magnitude: Magnitude<joule_per_mole>,
}

impl MolarEnthalpy {
  pub fn new(name: String, symbol: String, value: f64, unit: joule_per_mole) -> Self {
      let magnitude = Magnitude::new(name, symbol, value, unit);
      MolarEnthalpy { magnitude }
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


  // Mutable access.
  fn name_mut(&mut self) -> &mut String {
    &mut self.magnitude.name
  }

  fn value_mut(&mut self) -> &mut f64 {
    &mut self.magnitude.value
  }

}


pub struct DryGasEnthalpy {
  pub magnitude: Magnitude<joule_per_mole>,
}


pub struct MoistureGasEnthalpy {
  pub magnitude: Magnitude<joule_per_mole>,
}


pub struct SaturatedGasEnthalpy {
  pub magnitude: Magnitude<joule_per_mole>,
}
