
extern crate greetings;

// note incorrect error from linter about lang module
// maybe because it is just a folder
use greetings::lang::{english, french};
// here again while this is not folder
use greetings::random;

fn main() {
  println!("Hello, world!");
  let g = english::hello();
  println!("English says {}", g);
  println!("French says {}", french::hello());
  println!("Random module says {}", random::says());
}
