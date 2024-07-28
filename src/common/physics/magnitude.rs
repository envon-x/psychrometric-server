// extern crate uom;

pub struct Magnitude<Unit> {
  pub name: String,
  pub value: f64,
  pub unit: Unit
}

impl<T> Magnitude<T> {
  pub fn new(name: String, value: f64, unit: T) -> Self {
    Magnitude {name, value, unit}
  }
}