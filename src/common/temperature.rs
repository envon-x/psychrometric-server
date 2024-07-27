use crate:: common::magnitude::Magnitude;

extern crate uom;
use uom::si::thermodynamic_temperature::kelvin;

pub enum Temperature {
  Thermodynamic(Magnitude<kelvin>),
  WetBulb(Magnitude<kelvin>),
  DewPoint(Magnitude<kelvin>),
  DryBulb(Magnitude<kelvin>),
  
}