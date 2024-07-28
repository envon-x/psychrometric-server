use crate::common::physics::magnitude::Magnitude;

extern crate uom;
use uom::si::molar_energy::joule_per_mole;

pub trait Enthalpy {
  fn magnitude(&self) -> &Magnitude<joule_per_mole>;
}



pub struct DryGasEnthalpy {
  pub magnitude: Magnitude<joule_per_mole>,
}

impl Enthalpy for DryGasEnthalpy {
  fn magnitude(&self) -> &Magnitude<joule_per_mole> {
      &self.magnitude
  }
}



pub struct MoistureGasEnthalpy {
  pub magnitude: Magnitude<joule_per_mole>,
}

impl Enthalpy for MoistureGasEnthalpy {
  fn magnitude(&self) -> &Magnitude<joule_per_mole> {
      &self.magnitude
  }
}




pub struct SaturatedGasEnthalpy {
  pub magnitude: Magnitude<joule_per_mole>,
}

impl Enthalpy for SaturatedGasEnthalpy {
  fn magnitude(&self) -> &Magnitude<joule_per_mole> {
      &self.magnitude
  }
}
