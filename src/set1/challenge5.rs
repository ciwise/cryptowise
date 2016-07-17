//! This challenge requests that we encrypt a specified text string in plain-text using a repeating key
//! XOR operation using "ICE" as the three bytes that will be repeated beneath the plain-text from left
//! to right. ICE equates to 49,43, and 45 in base16(hex) respectively. The plain-text string must be first
//! converted to a hex string and then the 494345494345... sequence XOR'ed with it to produce the encrypted
//! hexadecimal string. The encrypted hex string is given in the cryptopals challenge.
//!
//! # Examples
//!
//!

use rustc_serialize::hex::ToHex;
use set1::challenge2::xor;

/// This function takes a static string argument and converts it
/// to hex returning String.
pub fn static_to_hex(input: &str) -> String {
  return input.to_string().as_bytes().to_hex();
}


#[cfg(test)]
mod tests {

  use super::*;


  #[test]
  fn test_static_string_to_hex() {
    let plaintxt = "My name is David L. Whitehurst";
	let hex = static_to_hex(plaintxt);
    assert_eq!("4d79206e616d65206973204461766964204c2e2057686974656875727374", 
	   &hex);
  }
  
  #[test]
  fn test_static_challenge_string_to_hex() {
    let plaintxt = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
	let hex = static_to_hex(plaintxt);
    assert_eq!("4275726e696e672027656d2c20696620796f752061696e277420717569636b20616e64206e696d626c650a4920676f206372617a79207768656e2049206865617220612063796d62616c", 
	   &hex);
	
  }
  
  #[test]
  fn test_repeating_xor() {
    use set1::challenge2::xor;
	use rustc_serialize::hex::ToHex;
	
	let plain_text_hex = "4275726e696e672027656d2c20696620796f752061696e277420717569636b20616e64206e696d626c650a4920676f206372617a79207768656e2049206865617220612063796d62616c";
	           let key = "4943454943454943454943454943454943454943454943454943454943454943454943454943454943454943454943454943454943454943454943454943454943454943454943454943";
    let result = xor(plain_text_hex, key);
	assert_eq!("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",result.to_hex());
  }
  // Burning 'em, if you ain't quick and nimble
  // I go crazy when I hear a cymbal
} // end mod tests
