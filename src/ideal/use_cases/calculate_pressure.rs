extern crate uom;
use uom::si::pressure::pascal;
use uom::si::volume::cubic_meter;
use uom::si::thermodynamic_temperature::kelvin;

use crate::common::physics::magnitude::Magnitude;
use crate::common::physics::thermo::eos::EquationOfState;


pub fn calculate_pressure(
    eos: &dyn EquationOfState, 
    molar_volume: &Magnitude<cubic_meter>, 
    temperature: &Magnitude<kelvin>) -> 
    Magnitude<pascal> {
    eos.calculate_pressure(molar_volume, temperature)
}