#[cfg(test)]
mod tests {
  extern crate greetings;
  use greetings::lang::english;
  // use greetings::english;
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
  #[test]
  fn english_hello(){
    assert_eq!(english::hello(),"Hello!");
  }
}