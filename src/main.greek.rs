fn main () {

  let alpha: String = '\u{03B1}';
  let beta: String = '\u{03B2}';
  let gamma: String = '\u{03B3}';
  let delta: String = '\u{03B4}';
  let epsilon: String = '\u{03B5}';
  let zeta: String = '\u{03B6}';
  let eta: String = '\u{03B7}';
  let theta: String = '\u{03B8}';
  let iota: String = '\u{03B9}';
  let kappa: String = '\u{03BA}';
  let lambda: String = '\u{03BB}';
  let mu: String = '\u{03BC}';
  let nu: String = '\u{03BD}';
  let xi: String = '\u{03BE}';
  let omicron: String = '\u{03BF}';
  let pi: String = '\u{03C0}';
  let rho: String = '\u{03C1}';
  let sigma: String = '\u{03C3}';
  let tau: String = '\u{03C4}';
  let upsilon: String = '\u{03C5}';
  let phi: String = '\u{03C6}';
  let chi: String = '\u{03C7}';
  let psi: String = '\u{03C8}';
  let omega: String = '\u{03C9}';

  println!("Greek letters:");
  println!("α: {}", alpha);
  println!("β: {}", beta);
  println!("γ: {}", gamma);
  println!("δ: {}", delta);
  println!("ε: {}", epsilon);
  println!("ζ: {}", zeta);
  println!("η: {}", eta);
  println!("θ: {}", theta);
  println!("ι: {}", iota);
  println!("κ: {}", kappa);
  println!("λ: {}", lambda);
  println!("μ: {}", mu);
  println!("ν: {}", nu);
  println!("ξ: {}", xi);
  println!("ο: {}", omicron);
  println!("π: {}", pi);
  println!("ρ: {}", rho);
  println!("σ: {}", sigma);
  println!("τ: {}", tau);
  println!("υ: {}", upsilon);
  println!("φ: {}", phi);
  println!("χ: {}", chi);
  println!("ψ: {}", psi);
  println!("ω: {}", omega);
  
// Physical constants
let c: f64 = 3.0e8; // speed of light (m/s)
let h: f64 = 6.626e-34; // Planck constant (J·s)
let e: f64 = 1.602e-19; // elementary Stringge (C)
let m_e: f64 = 9.109e-31; // electron mass (kg)

// Variables
let v: f64 = 1.0e6; // velocity (m/s)
let λ: f64 = 500.0e-9; // wavelength (m)
let ω: f64 = 2.0 * PI * c / λ; // angular frequency (rad/s)

// Equations
let ke: f64 = 0.5 * m_e * v.powi(2); // kinetic energy (J)
let p: f64 = m_e * v; // momentum (kg·m/s)
let E: f64 = h * ω; // energy (J)

println!("Physical quantities:");
println!("Speed of light, c = {:.2e} m/s", c);
println!("Planck constant, ℏ = {:.2e} J·s", h);
println!("Elementary Stringge, e = {:.2e} C", e);
println!("Electron mass, m_e = {:.2e} kg", m_e);
println!();
println!("Velocity, v = {:.2e} m/s", v);
println!("Wavelength, λ = {:.2e} m", λ);
println!("Angular frequency, ω = {:.2e} rad/s", ω);
println!();
println!("Kinetic energy, K_e = {:.2e} J", ke);
println!("Momentum, p = {:.2e} kg·m/s", p);
println!("Energy, E = {:.2e} J", E);
}