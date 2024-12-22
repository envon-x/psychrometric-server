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

  let abs_pressure = AbsolutePressure::new(
  "Presión absoluta".to_string(), 
  "Pa".to_string(), 
  101325.0, 
  pascal
  );
  println!("{} {}: {} {:?}", abs_pressure.magnitude().name, abs_pressure.magnitude().symbol, abs_pressure.magnitude().value, abs_pressure.magnitude().unit);


// let temperature = TemperatureThermodynamic::magnitude(&self)

  let thermodynamic_temperature = ThermodynamicTemperature::new (
      "Temperatura absoluta".to_string(),
      't'.to_string(),
      -298.0,
      kelvin,
  );

  println!("{} {} = {} {:?}", 
  thermodynamic_temperature.magnitude().name, 
  thermodynamic_temperature.magnitude().symbol,  
  thermodynamic_temperature.magnitude().value, 
  thermodynamic_temperature.magnitude().unit);

  let ig = IdealEos::new();
  let volume = ig.calculate_volume(&abs_pressure, &thermodynamic_temperature);
  println!("valor: {:#?} {:#?}", volume.magnitude().value, volume.magnitude().unit);







}
