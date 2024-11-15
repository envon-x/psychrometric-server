// use std::slice;

// use core::slice;

// pub fn as_slice(&self) ->&[T] {
//   self
// }


/**********
  Code like a pro in Rust 2022.pdf
 */
fn main() {

  // In the code above, we’ve initialized a byte array, containing 64 elements, 
  // all of which are zero. 0u8 is shorthand for an unsigned integral type, 
  // 8 bits in length, with a value of 0. 0 is the value, and u8 is the type.
  let array = [0u8; 64]; // The type signature here is [u8; 64], an array, initialized with zeroes.


  // On the second line, we’re borrowing the array as a slice. Up to now, this 
  // isn’t particularly interesting. You can do some slightly more interesting 
  // things with slices, such as borrowing
  let slice = &array;

  // Splits and borrows a slice twice, destructuring it into two separate non-
  // overlapping sub-slices The code above is calling the split_at() function, 
  // which is part of Rust’s core library a
  let (first_half, second_half) = slice.split_at(32);

  println!(
    "first_half.len() ={} second_half.len()={}",
    first_half.len(),
    second_half.len()
  );

  let word_list = "one, two, three, four, five";
  for word in word_list.split(','){
    println!("word: {}", word);
  }

  println!("\nAquí empieza todo lo que tiene que ver con EOS\n");

  

}