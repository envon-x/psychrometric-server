extern crate uom;
use uom::si::molar_volume::cubic_meter_per_mole;
use uom::si::pressure::pascal;
use uom::si::thermodynamic_temperature::kelvin;

use crate::common::physics::magnitude::Magnitude;
use crate::common::physics::thermo::eos::EquationOfState;
use crate::common::physics::thermo::molar_volume::Volume;

// pub fn calculate_volume(
//     eos: &dyn EquationOfState, 
//     pressure: &Magnitude<pascal>, 
//     temperature: &Magnitude<kelvin>
// ) -> Volume {
//     eos.calculate_volume(pressure, temperature)
// }