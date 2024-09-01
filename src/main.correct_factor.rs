use virial::domain::pressure_correction_factor::calculate_ln_f;

mod virial;
mod common; 

fn main() {
  // Define your parameters
  let kappa = 0.5;
  let p_v = 1.0;
  let p_t = 2.0;
  let r = 8.314;      // universal gas constant in J/(mol·K)
  let t = 298.15;     // temperature in Kelvin
  
  let x_gs = 0.1;
  
  let b_gg = 1.0;
  let b_gv = 1.0;
  
  let c_ggv = 1.0;
  let c_gvv = 1.0;
  let c_vvv = 1.0;
  let b_vv = 1.0;

  // Call the function
  let ln_f = calculate_ln_f(kappa, p_v, p_t, r, t, x_gs, b_gg, b_gv, c_ggv, c_gvv, c_vvv, b_vv);

  // Print the result
  println!("The value of ln f is: {}", ln_f);

  

}
