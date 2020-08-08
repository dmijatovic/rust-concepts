
struct Person{
  name:String,
}

// it needs to specify lifetime of borrowed struct,
// we use ' and generic indication
// #[derive(Debug)]
struct Company<'z>{
  name: String,
  ceo: &'z Person,
}

pub fn main(){
  // let x: &'static u8 = 5;
  let boss = Person {name:String::from("Elon Musk")};
  let tesla = Company{name:String::from("Tesla"),ceo: &boss};

  println!("Company CEO is: {}", tesla.ceo.name);
}