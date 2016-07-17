// Copyright 2016 params Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! This challenge module requires you to devise a way to score a response that's obtained
//! by doing an XOR operation in reverse. A plain-text string is XOR'ed with a single
//! character key. The cryptopal website is a little unclear about how this character
//! is used. The hex string given in the challenge is the result of the secret message
//! and the single key (2 char byte represenation) repeated 34 times beneath or across
//! the hex string length. The answer to this challenge is given by running the tests
//! like so:
//!
//! $ cargo test -- --nocapture
//!
//! # Test Notes
//! The syntax here is not correct for the nocapture option, however it does
//! work to allow the print statements to come with the test output.
//!
//! # Examples
//!
//! ``` 
//! let tmp = "Mary had a little lamb.";
//! assert_eq!(5, word_count(tmp));
//! ```
//!

  


use set1::challenge2::xor;
use std::str;
use std::iter;

/// This function returns a usize primitive with the integer count
/// of a provided std::string argument (statically addressable string)
/// 
/// # Examples
///
///
pub fn word_count(long_str: &str) -> usize {
   let v: Vec<&str> = long_str.split(' ').collect();
   return v.len();
}

/// This function provides a usize primitive (integer value) for the
/// best or most worded XOR result of this challenge. A loop is used
/// to *test* each single-repeatable hex character, representing the
/// alphabet (in decimal) between 65 and 123. Each decimal is converted
/// to a 2 char hex representation and then repeated 34 times creating
/// a string of similar lenght to the *encrypted* hex string given.
/// Each XOR produces what could be the secret message and the function
/// here does a word count. The XOR output with the most number of words
/// is kept and when the loop is terminated, the function returns this 
/// *best* usize value.
/// 
/// For this challenge the best result proved 7 words. To find the
/// secret message you'll have to checkout this code and run the tests.
///
/// # Examples
///
/// 
pub fn single_char_key() -> usize {
    let mut best: usize = 0;
    let mut answer: usize = 0;
    let mut message = "".to_string();

    for x in 65..123 {
        let hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
            // let a = "58585858585858585858585858585858585858585858585858585858585858585858";
        let hexkey = format!("{:02x}",x);
        let bigkey = iter::repeat(hexkey).take(34).collect::<String>();
        let result = xor(hex_str,&bigkey);
         let s = match String::from_utf8(result) {
           Ok(v) => v,
	       Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
         };
	 let wc = word_count(&s);
	 if wc > best {
	   best = wc;
	   answer = x;
	   message = s;
	 }

    }
    println!("C3:The decimal key is: {}",answer);
    println!("C3:The secret message is: {}", message);
    return best; 
}


#[cfg(test)]
mod tests {

  use super::*;


  #[test]
  fn test_word_count() {
    assert_eq!(5, word_count("Mary had a little lamb."));

  }

  #[test]
  fn test_single_char_xor() {
     assert_eq!(7, single_char_key());
  }

} // end mod tests
