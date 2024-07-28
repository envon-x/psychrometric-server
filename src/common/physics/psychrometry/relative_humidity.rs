pub struct RelativeHumidity {
  value: f64,
}

impl RelativeHumidity {
  fn new(value: f64) -> Self {
    Self { value }
  }
}