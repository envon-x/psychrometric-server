use crate::common::physics::{magnitude::Magnitude, thermo::eos::EquationOfState};

extern crate uom;
use uom::si::pressure::pascal;
use uom::si::volume::cubic_meter;
use uom::si::thermodynamic_temperature::kelvin;


pub struct VirialEos {
    b: f64, // virial coefficient
}

impl VirialEos {
    pub fn new(b: f64) -> Self {
        VirialEos { b }
    }
}

impl EquationOfState for VirialEos {
  
    fn calculate_pressure(
        &self,
        molar_volume: &Magnitude<cubic_meter>,
        temperature: &Magnitude<kelvin>,
    ) -> Magnitude<pascal> {
        // Virial EOS formula: P = RT / V + B * P / RT
        let r = 8.314; // gas constant
        let pressure = r * temperature.value / molar_volume.value + self.b * r * temperature.value / molar_volume.value;
        Magnitude::new("presion absoluta".to_string(), pressure, pascal)
    }

    fn calculate_volume(
        &self,
        pressure: &Magnitude<pascal>,
        temperature: f64,
    ) -> Magnitude<cubic_meter> {
        // Virial EOS formula: V = RT / P - B
        let r = 8.314; // gas constant
        let volume = r * temperature / pressure.value - self.b;
        Magnitude::new("volumen".to_string(), volume, cubic_meter)
    }
}