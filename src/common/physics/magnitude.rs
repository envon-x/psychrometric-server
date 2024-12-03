// extern crate uom;

pub struct Magnitude<Unit> {
  pub name: String,    // Nombre de la magnitud
  pub symbol: String,  // El sýmbolo que representa a la magnitud
  pub value: f64,      // Valor numérico de la magnitud
  pub unit: Unit       // Unidad de la magnitud
}

impl<T> Magnitude<T> {
  pub fn new(
    name: String, 
    symbol: String, 
    value: f64, 
    unit: T) -> Self {
    Magnitude {
      name, 
      symbol, 
      value, 
      unit
    }
  }
}