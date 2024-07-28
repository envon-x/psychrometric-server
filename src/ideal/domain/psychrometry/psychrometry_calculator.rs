
// pub trait Psychrometry {
//   fn calc_TWetBulb_From_TDewPoint(&self, tdp: &TemperatureDewPoint, p: &Pressure) -> TemperatureWetBulb;
//   fn calc_TWetBulb_From_RelHumidity(&self, hr: &RelHumidity, p: &Pressure) -> TemperatureWetBulb;
// }

impl Psychrometry for TemperatureDryBulb {
  fn calc_TWetBulb_From_TDewPoint(&self, tdp: &TemperatureDewPoint, p: &Pressure) -> TemperatureWetBulb {
      // implementation of the calculation
      unimplemented!()
  }

  fn calc_TWetBulb_From_RelHumidity(&self, hr: &RelHumidity, p: &Pressure) -> TemperatureWetBulb {
      // implementation of the calculation
      unimplemented!()
  }
}