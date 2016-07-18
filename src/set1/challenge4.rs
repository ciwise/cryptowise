// Copyright 2016 params Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! This challenge module requires the reading of a sizeable file with 60 hex-character
//! lines. One of the lines is encrypted using single-character XOR. It's our job to
//! find that line. I will use the techniques learned in challenge-3 for an ultimate 
//! solution.
//!

use set1::challenge2::xor;
use set1::challenge3::word_count;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str;
use std::iter;

/// This function reads the data file line by line and then sends
/// the line to the wc_single_char_xor_encrypted_line function and 
/// if line produces more than 2 words using one of the single characters
/// the secret message is extracted and printed to the screen.
pub fn process_file() -> Result<(), io::Error> {
  let f = try!(File::open("src/resources/4.txt"));
  let file = BufReader::new(&f);
  let mut lineno: usize = 0;
  for line in file.lines() {
   lineno = lineno + 1;
   let hex_line = line.unwrap();
   if wc_single_char_xor_encrypted_line(&hex_line) > 2 { // 2 words in UTF-8 Error
	 println!("\nC4:Line no:{} - {}", lineno, hex_line);
	 get_message(&hex_line);
   }
   //println!("{}",l);

  }
  Ok(())
}

/// This function is used when the process_file function has found a line with
/// at least 3 words. If a line contains two words, it may be "UTF8-8 Error" and
/// we don't want that line or message. The same loop is run again for 0 to 255
/// testing each single char key with the XOR operation and it prints the best
/// matching plain-text message to the console (using -- --nocapture)
pub fn get_message(hex_line_60: &str) {
    let mut best: usize = 0;
    let mut answer: usize = 0;
    let mut message = "".to_string();

    for x in 0..255 {
        let hexkey = format!("{:02x}",x);
        let bigkey = iter::repeat(hexkey).take(30).collect::<String>();
        let result = xor(hex_line_60, &bigkey);
		let utf8_error = "UTF-8 Error".to_string();
         let s = match String::from_utf8(result) {
           Ok(v) => v,
	       Err(_e) => utf8_error,
         };
	 let wc = word_count(&s);
	 if wc > best {
	   best = wc;
	   answer = x;
	   message = s;
	 }

    }

    println!("C4:The decimal key is: {}", answer);
    println!("C4:The secret message is: {}", message);
}
/// This finds and returns the highest word count it can find decrypting a hex string using XOR
/// and some single character key between 0 and 255 decimal. The hex string relating to the highest
/// word count could be used with get_message to see the de-crypted message and determine if it
/// is human-readable.
pub fn wc_single_char_xor_encrypted_line(hex_line_60: &str) -> usize {
    let mut best: usize = 0;

    for x in 0..255 {
        let hexkey = format!("{:02x}",x);
        let bigkey = iter::repeat(hexkey).take(30).collect::<String>();
        let result = xor(hex_line_60,&bigkey);
		let utf8_error = "UTF-8 Error".to_string();
         let s = match String::from_utf8(result) {
           Ok(v) => v,
	       Err(_e) => utf8_error,
         };
	 let wc = word_count(&s);
	 if wc > best {
	   best = wc;
	 }

    }

    return best; // highest word count using any error 
}

#[cfg(test)]
mod tests {

  use super::*;


  #[test]
  fn find_encrypted_line() {
    let result = process_file();
    // -- --no capture maybe?
    println!("{:?}", result);
  }
} // end mod tests

