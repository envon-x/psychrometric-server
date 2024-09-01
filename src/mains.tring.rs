fn print_String(s: String) {
  println!("print_String: {}", s);
}


fn print_str(s: &str) {
  println!("print_str: {}", s);
}
fn main() {
  let mut my_string = String::new();
  my_string = "Hola mundo".to_owned();

  let s_mutable = String::from("texto");
  print_String(s_mutable);
  print_str(&String::from("texto"));


  let s_immutable =  "Sou um texto inmutable";
  print_str(&String::from("Esto es un string sin haber declarado la variable"));
  print_str(&s_immutable);
  print_str("str passed directly");
}