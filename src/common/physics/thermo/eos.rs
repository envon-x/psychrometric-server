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
    molar_volume: &Magnitude<cubic_meter>, 
    temperature: &Magnitude<kelvin>
  ) -> Magnitude<pascal>;
  //  {

  //   // let p = z * R * temperature.
  //   _!NotI
  // }
  
  fn calculate_volume(
    &self, 
    pressure: &Magnitude<pascal>, 
    temperature: f64
  ) -> Magnitude<cubic_meter>;
}