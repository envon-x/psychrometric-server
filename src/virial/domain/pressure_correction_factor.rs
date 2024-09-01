/**
 * Pressure correction factor for real gases
 */
pub fn calculate_ln_f(
  kappa: f64,
  p_v: f64,
  p_t: f64,
  r: f64,
  t: f64,
  x_gs: f64,
  b_gg: f64,
  b_gv: f64,
  c_ggv: f64,
  c_gvv: f64,
  Cvvv: f64,
  b_vv: f64,
) -> f64 {
  let rt = r * t;
  let term1 = ((1.0 + kappa * p_v) * (p_t - p_v) - 0.5 * kappa * (p_t.powi(2) - p_v.powi(2))) / rt;
  let term2 = (1.0 - kappa * x_gs * p_t).ln();
  let term3 = - (2 * x_gs.powi(3) * (2.0 - 3.0 * x_gs) * p_t.powi(2)) / (rt.powi(2)) * b_gg * b_gv;
  let term4 = (x_gs.powi(2) / rt) * b_gg;
  let term5 = - (2.0 * x_gs.powi(2) * p_t / rt) * b_gg;
  let term6 = - ((p_t - p_v) / rt - (x_gs.powi(2) * p_t) / (rt.powi(2))) * b_gv;
  let term7 = (3.0 * x_gs / (2.0 * rt.powi(2))) * (1.0 + 2.0 * x_gs) * p_t.powi(2) * c_ggv;
  let term8 = - (3.0 * x_gs.powi(2) * p_t.powi(2)) / (rt.powi(2)) * c_gvv;
  let term9 = - ((1.0 + 2.0 * x_gs) * (1.0 - x_gs) * p_t.powi(2) - p_v.powi(2)) / (2.0 * rt.powi(2)) * Cvvv;
  let term10 = - (3.0 * x_gs * p_t.powi(2)) / (2.0 * rt.powi(2)) * b_gg.powi(2);
  let term11 = - (x_gs.powi(2) * (1.0 - 3.0 * x_gs) * (1.0 - x_gs) * p_t.powi(2)) / (rt.powi(2)) * b_gg * b_vv;
  let term12 = (6.0 * x_gs * (1.0 - x_gs) * p_t.powi(2)) / (rt.powi(2)) * b_vv * b_gv;
  let term13 = - (2.0 * x_gs * (1.0 - x_gs) * (1.0 - 3.0 * x_gs) * p_t.powi(2)) / (rt.powi(2)) * b_gv.powi(2);
  let term14 = (x_gs.powi(3) * p_t.powi(2)) / (rt.powi(2)) * c_ggv;
  let term15 = - ((p_v.powi(2) - (1.0 + 3.0 * x_gs) * (1.0 - x_gs).powi(3) * p_t.powi(2)) / (2.0 * rt.powi(2))) * b_vv.powi(2);

  term1 * b_gg + term2 + term3 + term4 + term5 + term6 + term7 + term8 + term9 + term10 + term11 + term12 + term13 + term14 + term15
}

fn calculate_ln_f_improved(
    kappa: f64,
    Pvap: f64,
    Ptotal: f64,
    R: f64,
    T: f64,
    V_hat_Ve: f64,
    k_H: f64,
    x_G_s: f64,
    B_GG: f64,
    B_GV: f64,
    B_VV: f64,
    C_GGV: f64,
    C_GVV: f64,
    C_VVV: f64,
    C_GGG: f64,
) -> f64 {
    let term1 = ((1.0 + kappa * Pvap) * (Ptotal - Pvap) - 0.5 * kappa * (Ptotal.powi(2) - Pvap.powi(2))) / (R * T) * V_hat_Ve;
    
    let term2 = (1.0 - k_H * x_G_s * Ptotal).ln() 
        - (2.0 * x_G_s.powi(3) * (2.0 - 3.0 * x_G_s) * Ptotal.powi(2)) / (R * T).powi(2) * B_GG * B_GV;
    
    let term3 = x_G_s.powi(2) / (R * T) * B_GG 
        - (2.0 * x_G_s.powi(2) * Ptotal) / (R * T) * B_GV 
        - ((Ptotal - Pvap) / (R * T) - (x_G_s.powi(2) * Ptotal) / (R * T)) * B_GG;
    
    let term4 = (3.0 * x_G_s.powi(2) / (2.0 * (R * T).powi(2))) * (1.0 + 2.0 * x_G_s) * Ptotal.powi(2) * C_GGV;
    
    let term5 = - (3.0 * x_G_s.powi(2) * (1.0 - x_G_s) * Ptotal.powi(2)) / (R * T).powi(2) * C_GVV;
    
    let term6 = - ((1.0 + 2.0 * x_G_s) * (1.0 - x_G_s).powi(2) * Ptotal.powi(2) - Pvap.powi(2)) / (2.0 * (R * T).powi(2)) * C_VVV;
    
    let term7 = - (3.0 * x_G_s.powi(4) * Ptotal.powi(2)) / (2.0 * (R * T).powi(2)) * B_GG.powi(2);
    
    let term8 = - (x_G_s.powi(2) * (1.0 - 3.0 * x_G_s) * (1.0 - x_G_s) * Ptotal.powi(2)) / (R * T).powi(2) * B_GG * B_VV;
    
    let term9 = (6.0 * x_G_s.powi(2) * (1.0 - x_G_s).powi(2) * Ptotal.powi(2)) / (R * T).powi(2) * B_VV * B_GV;
    
    let term10 = - (2.0 * x_G_s.powi(2) * (1.0 - x_G_s) * (1.0 - 3.0 * x_G_s) * Ptotal.powi(2)) / (R * T).powi(2) * B_GV.powi(2);
    
    let term11 = (x_G_s.powi(3) * Ptotal.powi(2)) / (R * T).powi(2) * C_GGG;
    
    let term12 = - (Pvap.powi(2) - (1.0 + 3.0 * x_G_s) * (1.0 - x_G_s).powi(3) * Ptotal.powi(2)) / (2.0 * (R * T).powi(2)) * B_VV.powi(2);
    
    term1 + term2 + term3 + term4 + term5 + term6 + term7 + term8 + term9 + term10 + term11 + term12
}
