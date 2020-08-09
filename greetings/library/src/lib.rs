
// Modules need to be public
// in order to be accessed from other projects
pub mod lang{
  pub mod english;
  pub mod french;
}

pub mod random;


// for test in VSC you you need Cargo.toml in the root
// Runing local test in the rust file
// See tests folder for test module
#[test]
fn english_hello(){
  assert_eq!("Hello!", lang::english::hello());
}

