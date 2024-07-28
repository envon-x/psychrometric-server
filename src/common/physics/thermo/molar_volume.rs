use crate::common::physics::magnitude::Magnitude;

extern crate uom;
use uom::si::molar_volume::cubic_meter_per_mole;

pub trait Volume {
  fn magnitude(&self) -> &Magnitude<cubic_meter_per_mole>;
}




pub struct DryGasVolume {
  pub magnitude: Magnitude<cubic_meter_per_mole>,
}

impl Volume for DryGasVolume {
  fn magnitude(&self) -> &Magnitude<cubic_meter_per_mole> {
      &self.magnitude
  }
}




pub struct MoistureGasVolume {
  pub magnitude: Magnitude<cubic_meter_per_mole>,
}

impl Volume for MoistureGasVolume {
  fn magnitude(&self) -> &Magnitude<cubic_meter_per_mole> {
      &self.magnitude
  }
}
