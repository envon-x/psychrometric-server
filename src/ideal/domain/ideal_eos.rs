use crate::common::physics::{magnitude::Magnitude, thermo::eos::EquationOfState};

extern crate uom;
use uom::si::pressure::pascal;
use uom::si::volume::cubic_meter;
use uom::si::thermodynamic_temperature::kelvin;

pub struct IdealEos {
  compressibility_factor: f64,
  universal_gas_constant: f64,
}

impl IdealEos {
  pub fn new() -> Self {
      IdealEos {
        compressibility_factor: 1.0,
        universal_gas_constant: 8.314,
      }
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
      let pressure = self.compressibility_factor * R * temperature.value / molar_volume.value;
      Magnitude::new("absolute pressure".to_string(), "p".to_string(), pressure, pascal)
  }

  fn calculate_volume(
      &self,
      pressure: &Magnitude<pascal>,
      temperature: &Magnitude<kelvin>,
  ) -> Magnitude<cubic_meter> {
      // formula: V = RT / P
      let R = 8.314; // gas constant
      let volume = z * R * temperature / pressure.value;
      Magnitude::new("volume".to_string(), "v".to_string(),  volume, cubic_meter)
  }
}
