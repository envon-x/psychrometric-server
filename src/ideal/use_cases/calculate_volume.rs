extern crate uom;
use uom::si::molar_volume::cubic_meter_per_mole;
use uom::si::pressure::pascal;
use uom::si::thermodynamic_temperature::kelvin;

use crate::common::physics::thermo::eos::EquationOfState;
use crate::common::physics::thermo::molar_volume::MolarVolume;
use crate::common::physics::thermo::pressure::AbsolutePressure;
use crate::common::physics::thermo::temperature::ThermodynamicTemperature;

pub fn calculate_volume(
    eos: &dyn EquationOfState, 
    pressure: &AbsolutePressure, 
    temperature: &ThermodynamicTemperature,
) -> MolarVolume {
    eos.calculate_volume(pressure, temperature)
}