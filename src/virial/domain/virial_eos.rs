use crate::common::constants::symbol;
use crate::common::physics::{magnitude::Magnitude, thermo::eos::EquationOfState};

extern crate uom;
use uom::si::pressure::pascal;
use uom::si::volume::cubic_meter;
use uom::si::thermodynamic_temperature::kelvin;

// mod common;

pub struct VirialEos {
    b: f64, // virial coefficient  
    second_virial_coeffcient: f64,
    thrid_virial_coefficient: f64,
    universal_gas_constant: f64,
}

impl VirialEos {
    pub fn new(b: f64) -> Self {
        VirialEos { 
            b 

        }
    }
}


impl EquationOfState for VirialEos {

  fn calculate_pressure(
    &self,
    z: f64, // factor de compresibilidad
    molar_volume: &Magnitude<cubic_meter>,
    temperature: &Magnitude<kelvin>,
  ) -> Magnitude<pascal> {
    // Virial EOS formula: P = RT / V + B * P / RT
    let r = 8.314; // gas constant
    let pressure_value = z * r * temperature.value / molar_volume.value + self.b * r * temperature.value / molar_volume.value;
    Magnitude::new(
      "absolute_pressure".to_string(), 
      symbol::SYMBOL_PRESSURE.to_string(), 
      pressure_value, 
      pascal
    )
  }

  fn calculate_volume(
    &self,
    z: f64,
    pressure: &Magnitude<pascal>,
    temperature: f64,
  ) -> Magnitude<cubic_meter> {
    let r = 8.314; // gas constant
    let volume_value = z * r * temperature / pressure.value - self.b;
    Magnitude::new(
      "molar_volume".to_string(), 
      symbol::SYMBOL_MOLAR_VOLUME.to_string(), 
      volume_value, 
      cubic_meter
    )
  }
}
