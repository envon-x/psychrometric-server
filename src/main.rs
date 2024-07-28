mod common; 

use common::physics::{
  magnitude::Magnitude, 
  thermo::temperature::{
    TemperatureDewPoint, 
    TemperatureDryBulb
  }
};
mod ideal;
use ideal::{domain::ideal_eos::IdealEos, use_cases::calculate_pressure::calculate_pressure};
extern crate uom;
use uom::si::volume::cubic_meter;
use uom::si::thermodynamic_temperature::kelvin;
use virial::domain::virial_eos::VirialEos;

mod virial;

fn main() {

  println!("\n============ DEVELOPING PSYCHROMETRY BACKEND  ============");

  let dry_bulb_temperature = TemperatureDryBulb {
    magnitude: Magnitude {
      name: "Dry bulb temperature".to_string(),
      value: 20.0,
      unit: kelvin,
    },
  };

  println!("Dry bulb temperature: {}", dry_bulb_temperature.magnitude.value);


  let dew_point_temperature = TemperatureDewPoint {
    magnitude: Magnitude {
      name: "Temperatura de rocio".to_string(),
      value: 30.0,
      unit: kelvin,
    },
  };

  println!("Temperatura de punto e rocio {}", dew_point_temperature.magnitude.value);


  println!("\n============ EOS  ============");
  let ideal_eos = IdealEos::new();
  let virial_eos = VirialEos::new(0.1); // example virial coefficient

  let molar_volume = Magnitude::new("molar volume".to_string(), 0.0224, cubic_meter);
  let temperature = Magnitude::new("absolute temperature".to_string(), 300.0, kelvin);

  let pressure_ideal = calculate_pressure(&ideal_eos, &molar_volume, &temperature);
  let pressure_virial = calculate_pressure(&virial_eos, &molar_volume, &temperature);

  println!("Ideal EOS pressure: {}", pressure_ideal.value);
  println!("Virial EOS pressure: {}", pressure_virial.value);

  // let rel_hum = calc_RelHumidity_From_TDewPoint(dry_bulb_temperature, dew_point_temperature);
  // println!("Relative humidity: {}", rel_hum.value);

    // let atmosphere_pressure = Press
}
