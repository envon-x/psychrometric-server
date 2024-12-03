extern crate uom;
use uom::si::{
  volume::cubic_meter, 
  thermodynamic_temperature::kelvin,
  pressure::pascal,
};

use crate::common::physics::magnitude::Magnitude;

pub trait EquationOfState {

  /// Permite calcular la presión del sistema en términos del sistema internacional (SI)
  /// z: coeficiente virial, adimensional
  /// molar_volume: Es el objeto Magnitud, cuyo valor es el volumen molar y en unidades m³/mol
  /// temperature: Es el objeto magnitud cuyo valor es la  temperatura en kelvin's, K
  ///
  /// return: El objeto Magnitud pressure (presión), calculado bajo el EOF que lo implementa en
  /// unidades, Pa.
  fn calculate_pressure(
    &self,
    molar_volume: &Magnitude<cubic_meter>, 
    temperature: &Magnitude<kelvin>
  ) -> Magnitude<pascal>;
    
  /// Permite calcular el volumen molar del sistema en términos del sistema internacional (SI), m³/mol
  /// z: coeficiente virial, adimensional
  /// pressure: Es el objeto Magnitud, cuyo valor es la presión y en unidades, Pa
  /// temperature: Es el objeto magnitud cuyo valor es la  temperatura en kelvin's, K
  ///
  /// return: El objeto Magnitud volumen (molar volume), calculado bajo el EOF que lo implementa en
  /// donde la unidad de la magnitd es, m³/mol.
  fn calculate_volume(
    &self,
    pressure: &Magnitude<pascal>, 
    temperature: &Magnitude<kelvin>
  ) -> Magnitude<cubic_meter>;
}
