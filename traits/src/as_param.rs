
use std::fmt::Debug;

// Traits defined as parameters

pub trait Shape{
  fn area(&self)->f64;
}

#[derive(Debug)]
pub struct Circle{
  pub radius: f64,
}

impl Shape for Circle {
  fn area(&self) -> f64 {
    self.radius * self.radius * std::f64::consts::PI
  }
}

#[derive(Debug)]
pub struct Square{
  pub x: f64,
}

impl Shape for Square {
  fn area(&self) -> f64 {
    self.x * self.x
  }
}

// using trait as parameter, also uses reference to shape
pub fn print_info(shape: &impl Shape){
  println!("Trait as param. The area is {}", shape.area())
}

// using 2 traits with + sign
// passed shape needs to inplement Shape and Debug traits)
// it can be written in 3 different ways:
// 1:
pub fn print_info_with_debug1(shape: &(impl Shape + Debug)){
  println!("Trait as param. The area is {}. The object is {:?}", shape.area(),shape)
}
// 2:
pub fn print_info_with_debug2<T:Shape+Debug>(shape: &T){
  println!("Trait as param. The area is {}. The object is {:?}", shape.area(),shape)
}
// 3:
pub fn print_info_with_debug3<T>(shape: &T)
  where T: Shape + Debug{
  println!("Trait as param. The area is {}. The object is {:?}", shape.area(),shape)
}