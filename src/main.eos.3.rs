mod common; 

use common::physics::{
  magnitude::Magnitude, 
  thermo::temperature::{
    Temperature, TemperatureDewPoint, TemperatureDryBulb
  }
};
mod ideal;
use ideal::{domain::ideal_eos::IdealEos, use_cases::calculate_pressure::calculate_pressure};
extern crate uom;
use peroxide::fuga::anyhow::Ok;
use uom::si::volume::cubic_meter;
use uom::si::thermodynamic_temperature::kelvin;
use virial::domain::{compresibility_factor, virial_eos::VirialEos};

mod virial;

fn main() {

  let dry_bulb_temperature = Magnitude::new("Dry_bulb_temperature".to_string(), "T".to_string(), 45.5, kelvin);

  let temperature = Magnitude::new(
    "absolute temperature".to_string(), 
    String::from("T"), 
    300.0, 
    kelvin
  );

  println!("\n============  EOS  =============");


  let ideal_eos = IdealEos::new();
  let molar_volume = Magnitude::new("molar volume".to_string(), "V".to_string(), 0.0224, cubic_meter);
  let temperature = Magnitude::new("absolute temperature".to_string(), "T".to_string(), 300.0, kelvin);
  
  let pressure_ideal = calculate_pressure(&ideal_eos, &molar_volume, &temperature);
  println!("Ideal EOS pressure:  {} {:?}", pressure_ideal.value, pressure_ideal.unit);

  let virial_eos = VirialEos::new(0.1); // example virial coefficient
  let pressure_virial = calculate_pressure(&virial_eos, &molar_volume, &temperature);
  println!("Virial EOS pressure: {}", pressure_virial.value);

  // let rel_hum = calc_RelHumidity_From_TDewPoint(dry_bulb_temperature, dew_point_temperature);
  // println!("Relative humidity: {}", rel_hum.value);

    // let atmosphere_pressure = Press

  
}
