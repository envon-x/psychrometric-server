extern crate uom;
use uom::si::pressure::pascal;
use uom::si::volume::cubic_meter;
use uom::si::thermodynamic_temperature::kelvin;

use crate::common::physics::magnitude::Magnitude;
use crate::common::physics::thermo::eos::EquationOfState;
use crate::common::physics::thermo::molar_volume::MolarVolume;
use crate::common::physics::thermo::pressure::AbsolutePressure;
use crate::common::physics::thermo::temperature::ThermodynamicTemperature;


pub fn calculate_pressure(
  eos: &dyn EquationOfState, 
  molar_volume: &MolarVolume, 
  temperature: &ThermodynamicTemperature
) -> AbsolutePressure {
  eos.calculate_pressure(molar_volume, temperature)
}