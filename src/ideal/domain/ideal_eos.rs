use crate::common::physics::{magnitude::Magnitude, thermo::eos::EquationOfState};

extern crate uom;
use uom::si::pressure::pascal;
use uom::si::volume::cubic_meter;
use uom::si::thermodynamic_temperature::kelvin;

pub struct IdealEos {}

impl IdealEos {
  pub fn new() -> Self {
      IdealEos {}
  }
}

impl EquationOfState for IdealEos {
  fn calculate_pressure(
      &self,
      molar_volume: &Magnitude<cubic_meter>,
      temperature: &Magnitude<kelvin>,
  ) -> Magnitude<pascal> {
      // formula: P = RT / V
      let R = 8.314; // gas constant
      let pressure = R * temperature.value / molar_volume.value;
      Magnitude::new("absolute pressure".to_string(), pressure, pascal)
  }

  fn calculate_volume(
      &self,
      pressure: &Magnitude<pascal>,
      temperature: f64,
  ) -> Magnitude<cubic_meter> {
      // formula: V = RT / P
      let R = 8.314; // gas constant
      let volume = R * temperature / pressure.value;
      Magnitude::new("volume".to_string(), volume, cubic_meter)
  }
}