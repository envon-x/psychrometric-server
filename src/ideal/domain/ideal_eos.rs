use crate::common::physics::thermo::molar_volume::MolarVolume;
use crate::common::physics::thermo::pressure::{AbsolutePressure, Pressure};
use crate::common::physics::thermo::temperature::ThermodynamicTemperature;
use crate::common::physics::{magnitude::Magnitude, thermo::eos::EquationOfState};

extern crate uom;
use uom::si::molar_volume::cubic_meter_per_mole;
use uom::si::pressure::pascal;
use uom::si::volume::cubic_meter;
use uom::si::thermodynamic_temperature::kelvin;

#[derive(Debug)]
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
    molar_volume: &MolarVolume,
    temperature: &ThermodynamicTemperature,
  ) -> AbsolutePressure {
    // formula: P = RT / V
    let r = self.universal_gas_constant;
    let pressure = self.compressibility_factor * r * temperature.magnitude().value / molar_volume.magnitude().value;

    AbsolutePressure::new(
      "absolute pressure".to_string(), 
      "p".to_string(), 
      pressure, 
      pascal
    )
  }
  
  /// Calculate volume for ideal gas
  /// formulae: V = zRT / P
  ///
  fn calculate_volume(
    &self,
    pressure: &AbsolutePressure,
    temperature: &ThermodynamicTemperature,
  ) -> MolarVolume {
    let r = self.universal_gas_constant;
    let z = self.compressibility_factor;
    
    let volume_value = z * r * temperature.magnitude().value / pressure.magnitude().value;


    // if volume_value <= 0.0 {
    //   panic!("Can't proceed with this volume_value: {}", volume_value)
    // }



    MolarVolume::new(
      "volume".to_string(), 
      "v".to_string(),  
      volume_value, 
      cubic_meter_per_mole
    )

  }
}



