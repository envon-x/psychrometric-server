mod common; 

use common::{magnitude::Magnitude, temperature::Temperature};
use uom::si::thermodynamic_temperature::kelvin;



fn main() {

    println!("============ INIT  ============");

    let wet_bulb_temperature = Temperature::WetBulb(Magnitude { name: "Temperatura de bulbo humedo".to_string(), value: 25.0, unit: kelvin });
    
    match wet_bulb_temperature {
        Temperature::WetBulb(mag) => println!("Wet temperature: {}: {} {:?}", mag.name, mag.value, mag.unit),
        _ => unreachable!(),
    }
    
    let dry_bulb_temperature = Temperature::DryBulb(Magnitude { name: "Temperatura de bulbo seco".to_string(), value: 30.0, unit: kelvin });
    match dry_bulb_temperature {
        Temperature::DryBulb(mag) => println!("Dry temperature: {}: {} {:?}", mag.name, mag.value, mag.unit),
        _ => unreachable!(),
    }

    // let atmosphere_pressure = Press
}
