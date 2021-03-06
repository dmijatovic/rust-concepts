
#[derive(Debug)]
pub struct Person{
  first_name:String,
  last_name: String,
  age:u16
}

impl Person{
  pub fn new(first_name:String,last_name:String,age:u16)->Self{
    Person{
      first_name,
      last_name,
      age
    }
  }
  pub fn repr(&self)->String{
    format!("{:?}", &self)
  }
}