// extern crate uom;

/**
 * Bauer, James Birk, Pamela Marks - Chemistry.pdf page: 93
 * /media/bon/MainStore/Biblioteca/Chemistry/Chemistry
 * 
 * Physical component of mixture mixture gases
 */
pub struct Compound {
  pub name: String,           // Molecular coumpound name
  pub symbol: String,         //
  pub molecular_weight: f64,
}

impl Compound {
  pub fn new(
    name: String,
    symbol: String,
    molecular_weight: f64, 
  ) -> Self {
    Compound {
      name,
      symbol,
      molecular_weight, 
    }
  }
}