//! This challenge is to perform  
//! an exclusive bitwise OR operation on two hexadecimal strings.
//!
//! # Examples
//!
//! ```
//! extern crate rustc_serialize;
//! extern crate cryptowise;
//! use cryptowise::set1::challenge2::xor;
//! use rustc_serialize::hex::ToHex;
//! fn main() {
//! let a = "1c0111001f010100061a024b53535009181c";
//! let b = "686974207468652062756c6c277320657965";
//! let result = xor(a,b);
//! assert_eq!("746865206b696420646f6e277420706c6179",result.to_hex());
//! }
//! ```
//!


/// This function takes two hex strings and does an exclusive
/// bitwise OR (XOR) operation returning a vector of unsigned
/// 8-bit integers. This function provides the resulting string
/// with the to_hex() function (rustc_serialize).
///
/// # Examples
///
/// ```
/// extern crate rustc_serialize;
/// extern crate cryptowise;
/// use cryptowise::set1::challenge2::xor;
/// use rustc_serialize::hex::ToHex;
/// fn main() {
/// let a = "1c0111001f010100061a024b53535009181c";
/// let b = "686974207468652062756c6c277320657965";
/// let result = xor(a,b);
/// assert_eq!("746865206b696420646f6e277420706c6179",result.to_hex());
/// }
/// ```
///

use rustc_serialize::hex::FromHex;

pub fn xor(a: &str, b: &str) -> Vec<u8> {
   return a.from_hex().unwrap().iter().zip(b.from_hex().unwrap().iter()).map(|(&x,&y)|x^y).collect();

}

#[cfg(test)]
mod tests {
  use super::*;
  use rustc_serialize::hex::ToHex;

  #[test]
  fn test_xor() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";
    let result = xor(a,b);
    assert_eq!("746865206b696420646f6e277420706c6179",result.to_hex());
  }
} // end mod tests

