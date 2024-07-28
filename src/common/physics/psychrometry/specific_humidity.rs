// Check if only a value or has a magnitude
pub struct SpecificHumidity {
  value: f64,
}

impl SpecificHumidity {
  fn new(value: f64) -> Self {
    Self { value }
  }
}