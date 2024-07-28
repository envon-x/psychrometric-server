use crate::common::physics::thermo::eos::EquationOfState;
extern crate uom;
use uom::si::pressure::pascal;
use uom::si::volume::cubic_meter;
use crate::common::physics::magnitude::Magnitude;

pub fn calculate_volume(eos: &dyn EquationOfState, pressure: &Magnitude<pascal>, temperature: f64) -> Magnitude<cubic_meter> {
    eos.calculate_volume(pressure, temperature)
}