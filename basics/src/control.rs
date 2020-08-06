

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
    println!("Loop array at {}", val);
  }
}

pub fn loop_range(start:u8,end:u8){
  for val in start..end{
    println!("Loop range at {}", val);
  }
}

pub fn match_cases(){
  let country_code:u16 = 44;

  let country_name:&str = match country_code{
    31 => "Netherlands",
    32 => "Belgium",
    44 => "UK",
    46 => "Sweden",
    1..=254 => "Other country not in my list in range 1 -245",
    _ => "High likely not a country code"
  };

  println!("{} country code is for {}", country_code,country_name);
}