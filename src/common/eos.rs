pub trait EquationOfState {

  fn calculate_z(&self, volume: f64, temperature: f64) -> f64;
  
  fn calculate_pressure(&self, volume: f64, temperature: f64) -> f64 {
      let z = self.calculate_z(volume, temperature);
      let r = 1.0; // Assuming R = 1 for simplicity
      z * r * temperature / volume
  }

  fn calculate_volume(&self, pressure: f64, temperature: f64) -> f64 {
      let z = self.calculate_z(pressure, temperature);
      let r = 1.0; // Assuming R = 1 for simplicity
      r * temperature / pressure / z
  }
}