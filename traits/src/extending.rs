
// Here we extend existing Rust structure with our custom trait
// and then implement our own method for trait functionality

pub trait Sumable<T>{
  fn sum(&self) -> T;
}

impl Sumable<i32> for Vec<i32>{
  fn sum(&self)->i32{
    let mut result = 0;
    for x in self{
      result+=x;
    }
    return result;
  }
}