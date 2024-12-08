// extern crate uom;

use super::{critical_props::{self, CriticalProps}, triple_point_props::{self, TriplePointProps}};

/**
 * Bauer, James Birk, Pamela Marks - Chemistry.pdf page: 93
 * /media/bon/MainStore/Biblioteca/Chemistry/Chemistry
 * 
 * Physical component of mixture mixture gases
 */
pub struct Compound {
  pub name: String,           // Molecular compound name
  pub cas_number: String,
  pub formula: String,         //
  pub molecular_weight: f64,
  pub critical_props: CriticalProps,
  pub triple_point_props: TriplePointProps, 
}

impl Compound {
  pub fn new(
    name: String,
    cas_number: String,
    symbol: String,
    molecular_weight: f64,
    critical_props: CriticalProps, 
    triple_point_props: TriplePointProps,
  ) -> Self {
    Compound {
      name,
      cas_number,
      formula: symbol,
      molecular_weight, 
      critical_props,
      triple_point_props
    }
  }
}