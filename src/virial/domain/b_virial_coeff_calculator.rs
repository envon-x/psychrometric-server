use super::virial_eos::{self, VirialEos};

pub trait TsonopoulosMethod {
  fn b_coefficient(
    &self,
    absolute_temperature: f64,
    critical_temperature: f64,
    critical_pressure: f64,
    w: f64,
    a: f64,
    b: f64,
  ) -> f64;
}

impl TsonopoulosMethod for VirialEos {
    
    fn b_coefficient(
      &self,
      absolute_temperature: f64,
      critical_temperature: f64,
      critical_pressure: f64,
      w: f64,
      a: f64,
      b: f64,
    ) -> f64 {
      let r = 8.314;
      let t_red = absolute_temperature / critical_temperature;
      let f0 = 0.1445 - 0.330 / t_red - 0.1385 / t_red.powi(2) - 0.0121 / t_red.powi(3) - 0.000607 / t_red.powi(8);
      let f1 = 0.0637 + 0.331 / t_red.powi(2) - 0.423 / t_red.powi(3) - 0.008 / t_red.powi(8);
      let f2: f64= 1 / t_red.powi(6);
      let f3: f64= -1 / t_red.powi(8);
  
      (r * critical_temperature/critical_pressure) * (f0 + w * f1 + a * f2 + b * f3)
  }

}