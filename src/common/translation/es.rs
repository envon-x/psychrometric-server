lazy_static::lazy_static! {
  pub static ref T: std::collections::HashMap<&'static str, &'static str> = [
    ("pressure", "presión"),
    ("volume", "volumen"),    
    ("temperature", "temperatura"),
    ("Relative Humididty", "Hu")
  ].iter().cloned().collect();
}