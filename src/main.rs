use common::physics::thermo::{
  pressure::{
    self, 
    AbsolutePressure
  }, 
  temperature::{
  Temperature, 
  ThermodynamicTemperature
  }
};
use ideal::domain::ideal_eos::{self, IdealEos};
use uom::si::{pressure::bar, thermodynamic_temperature::kelvin};
use crate::common::physics::magnitude::Magnitude;
use common::physics::thermo::{
  eos::EquationOfState, 
  temperature::TemperatureDewPoint
};
use uom::si::pressure::pascal;

mod common;
// mod ideal;
mod ideal;

fn main() {
  println!(":::::::::::::::::::::::::: Thermo ::::::::::::::::::::::::::\n");

  // let pressure = AbsolutePressure::new(
        // String::from("Absolute Pressure"),
        // String::from("Pa"),
        // 101325.0,
    // );

    // Usando uom para obtener una representación más precisa
    // println!("Pressure: {} {} = {} {:?}", pressure.magnitude().name, pressure.magnitude().symbol, pressure.magnitude().value, pressure.magnitude().unit);

  let abs_pressure = AbsolutePressure::new(
  "Presión absoluta".to_string(), 
  "Pa".to_string(), 
  101325.0, 
  pascal
  );
  println!("{} {}: {} {:?}", abs_pressure.magnitude().name, abs_pressure.magnitude().symbol, abs_pressure.magnitude().value, abs_pressure.magnitude().unit);


// let temperature = TemperatureThermodynamic::magnitude(&self)

  let dew_point_temperature = TemperatureDewPoint::new (
      "Temperatura absoluta".to_string(),
      't'.to_string(),
      298.0,
      kelvin
  );

  println!("{} {} = {} {:?}", 
  dew_point_temperature.magnitude().name, 
  dew_point_temperature.magnitude().symbol,  
  dew_point_temperature.magnitude().value, 
  dew_point_temperature.magnitude().unit);

  let ig = IdealEos::new();
  println!("valor: {:#?}", ig.calculate_volume(&abs_pressure, &dew_point_temperature).magnitude().value);
}