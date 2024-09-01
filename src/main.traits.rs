trait Greete {
  fn say_hello(&self) {
    println!("Hello!");
  }
}

struct Student {
  education: i32, // eduaction years
}

struct Teacher {
  education: i32,
  teaching: i32,
}

impl Greete for Student {
    
}

impl Greete for Teacher {
 fn say_hello(&self) {
  println!("Hello, I am teacher Zhang!");
 }  
}

// Genéricos con restriccion
fn outer_say_hello<T: Greete> (t: &T) {
  t.say_hello();
}

fn main() {
  let s = Student {
    education: 3,
  };

  let t = Teacher {
    education: 5,
    teaching: 2,
  };

  outer_say_hello(&s);
  outer_say_hello(&t);
}
