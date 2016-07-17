//! This challenge is to convert a hexadecimal string respresention and 
//! prove a conversion to base 64 equates to a translation that's given.
//!
//! The string:
//!
//! 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
//!
//! Should produce:
//!
//! SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
//!
//! # Examples
//!
//! ```
//! use cryptowise::set1::challenge1::hex_to_base64;
//! assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
//! hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
//! ```
//!

/// This function converts a hex string to base64 string representation
///
/// # Examples
///
/// ```
/// use cryptowise::set1::challenge1::hex_to_base64;
/// assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
/// hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
/// ```
///

use rustc_serialize::hex::FromHex;
use rustc_serialize::base64::{ToBase64, STANDARD}; 

pub fn hex_to_base64(hex: &str) -> String {
   return hex.from_hex().unwrap().to_base64(STANDARD);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hex_tobase64() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",hex_to_base64(&input));  
  }

} // end mod tests
