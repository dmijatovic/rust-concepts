
use std::num::ParseIntError;

fn divide_o(x:i32,y:i32)->Option<i32>{
  if y == 0 {
    None
  } else {
    Some(x/y)
  }
}

// fn divideR(x:i32,y:i32)->Result<i32,ParseIntError>{
//   if y == 0 {
//     panic!("Cannot divide by 0")
//   } else {
//     Ok(x/y)
//   }
// }

fn divide_r(x:i32,y:i32)->Result<i32,ParseIntError>{
  Ok(Ok(x/y))?
}


pub fn use_option(){
  let x:i32=6;
  let y:i32=2;

  let result = divide_o(x,y);

  match result{
    Some(z)=>println!("{}/{}={}", x,y,z),
    None => println!("This did not worked")
  }
}

pub fn use_result(){
  let x:i32=6;
  let y:i32=0;

  let result = divide_r(x,y);

  match result{
    Ok(z)=>println!("{}/{}={}", x,y,z),
    Err(e) => println!("Failed with: {}",e)
  }
}