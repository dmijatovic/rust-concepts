
// using return keywoard
pub fn return1(x:i32,y:i32)->i64{
  return (x + y) as i64;
}

// last line in fn does not have ;
pub fn return2(x:i32,y:i32)->i64{
  (x + y) as i64
}

pub fn call_private_fn(){
  private_fn()
}

fn private_fn(){
  println!("This is private function and can be called from module only")
}
