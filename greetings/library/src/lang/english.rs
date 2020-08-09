//! This is module documentation.
//! I can use `markrown` here.
//! In addition, I can have examples.
//!
//! To gerate documentation execute `crate doc`
//!
//! ## Examples
//!
//! ```
//! use greeting::english::{hello};
//! println!("{}",hello());
//!
//! ```


// Functions need to be public in order
// to be accessed from other modules and apps
// Documenting with ///

/// hello function will return hello greeting.
/// One can use `markdown` syntax for documentation
pub fn hello()->String{
  let hel = "Hello!".to_string();
  return hel;
}

/// googbye fn returns Goodbye! string in english.
pub fn goodbye()->String{
  String::from("Goodbye!")
}