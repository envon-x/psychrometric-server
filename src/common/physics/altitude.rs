use crate::common::physics::magnitude::Magnitude;

extern crate uom;
use uom::si::length::meter;

pub trait Altitude {
  fn magnitude(&self) -> &Magnitude<meter>;
}

pub struct SeaLevelAltitude {
  pub magnitude: Magnitude<meter>,
}

impl Altitude for SeaLevelAltitude {
  fn magnitude(&self) -> &Magnitude<meter> {
      &self.magnitude
  }
}
