extern crate uom;
use uom::si::{
  volume::cubic_meter, 
  thermodynamic_temperature::kelvin,
  pressure::pascal,
};

use crate::common::physics::magnitude::Magnitude;

pub trait EquationOfState {

  fn calculate_pressure(
    &self,
    z: f64,
    molar_volume: &Magnitude<cubic_meter>, 
    temperature: &Magnitude<kelvin>
  ) -> Magnitude<pascal>;

  fn calculate_volume(
    &self,
    z: f64,
    pressure: &Magnitude<pascal>, 
    temperature: f64
  ) -> Magnitude<cubic_meter>;
}