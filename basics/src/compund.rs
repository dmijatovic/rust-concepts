
pub fn tuple(){
  let x = (34,3.5,1000033003, true, String::from("Can have all types"));
  println!("Tuple x {:?}",x);
}

pub fn array(){
  let x =  [34, 35, 1, 4];
  println!("Array fixed size and same type x {:?}",x);
  println!("Array value at position x[0]={}", x[0]);
}