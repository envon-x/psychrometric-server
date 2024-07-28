mod common; 

use common::physics::{
  magnitude::Magnitude, 
  thermo::temperature::{
    TemperatureDewPoint, 
    TemperatureDryBulb
  }
};
use uom::si::thermodynamic_temperature::kelvin;



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



  // let rel_hum = calc_RelHumidity_From_TDewPoint(dry_bulb_temperature, dew_point_temperature);
  // println!("Relative humidity: {}", rel_hum.value);

    // let atmosphere_pressure = Press
}
