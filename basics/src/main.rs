#![allow(dead_code)]

mod mutable;
mod scalar;
mod compund;
mod function;
mod control;
mod structs;
mod traits;
mod errors;
mod options;
mod generics;

fn mutable(){
  mutable::muted();
}

fn scalar(){
  scalar::variables();
  scalar::decimal();

}

fn compund(){
  compund::tuple();
  compund::array();

  //returned touple can be destrutured
  let (sum,mul) = compund::destructure();
  println!("Destructured touple: {} is sum and {} is mul", sum,mul);
}

fn function(){
  println!("Fn1 returning value 4+4={}",
      function::return1(4, 4));
  println!("Fn2 returning value 3+3={}",
      function::return2(3, 3));
  function::call_private_fn();
}

fn control(){
  control::if_then_else(5);
  control::if_then_else(25);
  control::if_then_else(50);

  control::loop_it(7);
  control::loop_while(5);
  control::loop_for([1,2,3,4,5]);
  control::loop_range(1,11);

  control::match_cases();
}

fn structs(){
  let p = structs::Person::new(
    "Dusan".to_string(),
    "Mijatovic".to_string(),
    50);

  println!("{}", p.repr());
}

// you need to import trait
// in order to be able to use it
use traits::Represent;

fn traits(){
  let p = traits::Person::new(
    "Dusan".to_string(),
    "Mijatovic".to_string(),
    50);
  println!("{}", p.repr());
}

fn errors(){
  // standard Result error
  errors::file_open_error();
  // using expect wil panic on error
  errors::file_open_error2();
  // panic will stop program execution
  // note: this code will not be executed
  errors::panic_now("This is my error message".to_string());
}

fn main() {
  println!("Basic main starts!");

  // mutable();
  // scalar();
  // compund();
  // function();
  // control();
  // structs();
  // traits();

  // options::use_option();
  // options::use_result();

  generics::main();

  // errors();
}
