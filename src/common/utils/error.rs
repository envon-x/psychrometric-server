
use std::io;
#[derive(Debug)]
pub enum NumberError {
  NegativeNumber,
  Infinity,
}

// Function that returns a Result
