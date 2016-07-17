// Copyright 2016 params Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! This challenge requires use to break a repeating-key XOR ("Vigenere"). This will be the
//! qualifying challenge for the rest of the challenges. It will be difficult. The claim is
//! that people know how to break this but less people can actually break it.
//!
//! An encrypted file is given as data. The lines have been base64'd after being encrypted.
//! The encryption is repeating XOR.

/// This function accepts two static hex strings of equal length and 
/// returns a hamming_distance or weight of usize (integer)
use set1::challenge2::xor;

pub fn hamming_distance(s1: &str, s2: &str) -> usize {
	// cat -- car
	// 636174 -- 636172
	if s1.len() != s2.len() {
		panic!("Undefined for static strings of unequal length: {}", -1);
	}
	let mut distance: usize = 0;
	let my_vector = xor(s1,s2);
         for b in my_vector {
			 if b != 0 {
				distance = &distance + 1;
			 }
         }
	return distance;
}

pub fn length_in_bytes(s1: &str) -> usize{
	let mut length: usize = 0;
	let bytes = s1.as_bytes();
    for _ in bytes {
       length = &length + 1;
    }
    return length;     	
}

#[cfg(test)]
mod tests {

  use super::*;


  #[test]
  fn test_hamming_distance() {
    let result = hamming_distance("636274","616172");
    println!("The hamming-distance is {}", result);
  }
  
  #[test]
  #[ignore]
  fn test_unmatched_len_strings() {
  	let result = hamming_distance("63627471", "616172");
  	println!("The hamming-distance is {}", result);
  }
} // end mod tests
