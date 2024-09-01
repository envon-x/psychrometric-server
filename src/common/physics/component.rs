use crate::common::chemistry::compound::Compound;

/**
 * Physical component of mixture mixture gases
 */
pub struct Component {
  pub compound: Compound,
  pub phase: Phase,
  pub composition: f64,
}

impl Component {
  pub fn new(
    compound: Compound,
    phase: String,
    composition: f64, 
  ) -> Self {
    Component {
      compound,
      phase,
      composition, 
    }
  }
}

pub enum Phase {
  VAPOR,
  GAS
}