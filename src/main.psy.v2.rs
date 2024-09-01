use uom::si::{pressure::pascal, ratio::part_per_thousand, thermodynamic_temperature::kelvin};

mod common; 

use common::physics::{
  magnitude::Magnitude, psychrometry::absolute_humidity::AbsoluteHumidity, thermo::{pressure::AbsolutePressure, temperature::{
    Temperature, TemperatureDewPoint, TemperatureDryBulb
  }}
};
use common::physics::psychrometry::calculator::PsychrometryCalculator;

mod ideal;
use ideal::domain::psychrometry_calculator;

mod virial;

fn main() {

  // let b_coeff = 
  
  let abs_hum = AbsoluteHumidity::new(
    "AbsoHum".to_string(), 
    "Y".to_string(),
    1.0,
    None
  );

  println!("Humedad absoluta: {}", abs_hum.symbol())

  let dbt = TemperatureDryBulb {
    magnitude: Magnitude {
      name: "Dry bulb temperature".to_string(),
      symbol: String::from("T_db"),
      value: 20.0,
      unit: kelvin,
    },
  };


  let tdp = TemperatureDewPoint {
    magnitude: Magnitude {
      name: "Temperatura de rocio".to_string(),
      symbol: String::from("T_db"),
      value: 30.0,
      unit: kelvin,
    },
  };

  // let hm = MoistureGasEnthalpy::new(
  //   name:
  // )

  let p = AbsolutePressure {
    magnitude: Magnitude {
        name: "Presion absoluta".to_string(),
        symbol: String::from("p"),
        value: 101325.0,
        unit: pascal,
    },
  };

  

  let calc_TWetBulb_from_TDewPoint_t = PsychrometryCalculator::calc_TWetBulb_from_TDewPoint(&tdp, &p);
}