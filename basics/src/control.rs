

pub fn if_then_else(x:i8){
  if x < 20 {
    println!("x {} is less than 20", x);
  } else if x >=20 && x <= 30{
    println!("x {} is between 20 and 30", x);
  } else {
    println!("x {} is > 30", x);
  }
}

pub fn loop_it(stop:u8){
  let mut x = 1;
  loop{
    println!("Loop it between {} and {}", x,stop);
    if x >= stop{
      break;
    }
    //increase by one
    x+=1;
  }
}

pub fn loop_while(stop:u8){
  let mut x = 1;
  while x < stop{
    println!("Loop it while x {} < stop {}", x,stop);
    //increase by one
    x+=1;
  }
}

pub fn loop_for(col:[u8;5]){
  for val in col.iter(){
    println!("Loopint array at {}", val);
  }
}