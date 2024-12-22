extern crate uom;
use uom::si::{
  molar_volume::cubic_meter_per_mole, 
  pressure::pascal, 
  thermodynamic_temperature::kelvin, 
  volume::cubic_meter
};

use crate::common::physics::magnitude::Magnitude;

use super::{
  molar_volume::{Volume, MolarVolume}, 
  pressure::{AbsolutePressure, Pressure}, 
  temperature::ThermodynamicTemperature
};

pub trait EquationOfState {

  /// Calcula la presión absoluta de un sistema termodinámico en términos del sistema internacional (SI)
  /// z: coeficiente virial, adimensional
  /// molar_volume: Es el objeto Magnitud, cuyo valor es el volumen molar y en unidades m³/mol
  /// temperature: Es el objeto magnitud cuyo valor es la  temperatura en kelvin's, K
  ///
  /// return: El objeto Magnitud pressure (presión), calculado bajo el EOF que lo implementa en
  /// unidades, Pa.
  fn calculate_pressure(
    &self,
    molar_volume: &MolarVolume, 
    temperature: &ThermodynamicTemperature
  ) -> AbsolutePressure;
    
  /// Calcula el volumen molar del sistema en términos del sistema internacional (SI), m³/mol
  /// z: coeficiente virial, adimensional
  /// pressure: Es el objeto Magnitud, cuyo valor es la presión y en unidades, Pa
  /// temperature: Es el objeto magnitud cuyo valor es la  temperatura en kelvin's, K
  ///
  /// return: El objeto Magnitud volumen (molar volume), calculado bajo el EOF que lo implementa en
  /// donde la unidad de la magnitd es, m³/mol.
  fn calculate_volume(
    &self,
    pressure: &AbsolutePressure, 
    temperature: &ThermodynamicTemperature,
  ) -> MolarVolume;
}
