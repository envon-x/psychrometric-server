use common::physics::thermo::{pressure::{self, AbsolutePressure}, temperature::{Temperature, TemperatureThermodynamic}};
use uom::si::pressure::pascal;

mod common;
// mod ideal;

fn main() {
  println!(":::::::::::::::::::::::::: Thermo ::::::::::::::::::::::::::\n");

  // let pressure = AbsolutePressure::new(
        // String::from("Absolute Pressure"),
        // String::from("Pa"),
        // 101325.0,
    // );

    // Usando uom para obtener una representación más precisa
    // println!("Pressure: {} {} = {} {:?}", pressure.magnitude().name, pressure.magnitude().symbol, pressure.magnitude().value, pressure.magnitude().unit);

  let abs_pressure = AbsolutePressure::new("Absolute Pressure".to_string(), "Pa".to_string(), 101325.0, pascal);
  println!("{} {}: {}", abs_pressure.magnitude().name, abs_pressure.magnitude().symbol, abs_pressure.magnitude().value);


// let temperature = TemperatureThermodynamic::magnitude(&self)
}