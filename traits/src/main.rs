
mod basic;
mod extending;
mod as_param;
mod into;

use basic::{Human,Dog,Animal};
use extending::{Sumable};
use as_param::{Circle, Square};
use into::{Person};

fn basic_trait(){
  // not directly clear that you need
  // to import Anima trait to indicate create is implemented
  let h = Human::create("Dusan");
  let d = Dog::create("Fokx");

  println!("{:?}", h);
  println!("{:?}", d);

  h.talk();
  d.talk();
}


fn extending_trait(){
  // custom trait on existing Rust infra
  let a = vec![1,2,3,4];
  // calling custom fn we implemented on vector based on trait Sumable
  println!("Custom method sum on vector using traits: {}",a.sum());

}

fn trait_as_param(){
  let c = Circle{radius:3.1};
  let s = Square{x:5f64};
  // println!("{}")

  as_param::print_info(&c);
  as_param::print_info(&s);

  as_param::print_info_with_debug1(&c);
  as_param::print_info_with_debug2(&s);
  as_param::print_info_with_debug3(&c);
}

fn into_trait(){
  let john = Person::new1("John");

  let name = "Jane";
  // this works because of Into type
  // defined on new methods
  let j = Person::new2(name);
  println!("Hi, {}",john.name);
  println!("Hi, {}",j.name);
}

fn main() {

  basic_trait();

  extending_trait();

  trait_as_param();

  into_trait();
}
