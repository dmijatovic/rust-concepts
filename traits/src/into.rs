
// Into traits alows conversion between types where that possible


pub struct Person{
  pub name: String,
}

impl Person{
  // pass parameter that can be converted to string
  // can be written in two ways
  // 1:
  pub fn new1<S: Into<String>>(name:S)->Person{
    Person{name: name.into()}
  }
  // 2:
  pub fn new2<S>(name:S)->Person
   where S: Into<String>{
    Person{name: name.into()}
  }
}