
pub trait Animal{
  //static method (because it does not uses &self)
  //generic creation method for structs taking start param
  //and returnig struct  implements this method
  fn create(name: &'static str) -> Self;
  fn name(&self)-> &'static str;
  fn talk(&self){
    //default implementation
    println!("{} cannot talk",self.name())
  }
}

#[derive(Debug)]
pub struct Human{
  name: &'static str
}

#[derive(Debug)]
pub struct Dog{
  name: &'static str
}

impl Animal for Human{
  fn create(name: &'static str)->Human{
    Human{
      name
    }
  }
  fn name(&self)-> &'static str{
    self.name
  }
  fn talk(&self){
    println!("Hello, I am {}.", self.name())
  }
}

impl Animal for Dog{
  fn create(name: &'static str) ->Dog{
    Dog{
      name
    }
  }
  fn name(&self)-> &'static str{
    self.name
  }
  // USES default talk method
  // fn talk(&self){
  //   println!("Hello, I am {}.", self.name())
  // }
}
