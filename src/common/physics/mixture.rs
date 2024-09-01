use super::component::Component;

pub struct Mixture {
  vapor: Component,
  gas: Component,
}

impl  Mixture {
  pub fn new(vapor: Component, gas: Component) -> Self {
    Mixture { vapor, gas }  
  }
}