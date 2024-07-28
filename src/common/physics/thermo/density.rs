use crate::common::physics::magnitude::Magnitude;

extern crate uom;
use uom::si::mass_density::kilogram_per_cubic_meter;

pub trait Density {
  fn magnitude(&self) -> &Magnitude<kilogram_per_cubic_meter>;
}



pub struct DryGasDensity {
  pub magnitude: Magnitude<kilogram_per_cubic_meter>,
}

impl Density for DryGasDensity {
  fn magnitude(&self) -> &Magnitude<kilogram_per_cubic_meter> {
      &self.magnitude
  }
}



pub struct MoistureGasDensity {
  pub magnitude: Magnitude<kilogram_per_cubic_meter>,
}

impl Density for MoistureGasDensity {
  fn magnitude(&self) -> &Magnitude<kilogram_per_cubic_meter> {
      &self.magnitude
  }
}

