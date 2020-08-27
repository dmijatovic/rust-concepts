use std::io;

fn main() {
  println!("Hello, world! Give me your weight on earth: ");

  //get user input from console
  let mut str_weight = String::new();

  //unwrap will put value or raise error (if any)
  io::stdin().read_line(&mut str_weight).unwrap();

  // trim and parse string to f32, note "turbofish" on parse
  // indicating we want to parse string as f32
  // let user_weight = str_weight.trim().parse::<f32>().unwrap();
  // or we can tell compiler that user_weights need to be f32
  let user_weight:f32 = str_weight.trim().parse().unwrap();


  let mw = calculate_mars_weight(user_weight);

  println!("Your weight on Mars is: {} kg", mw);
}


fn calculate_mars_weight(earth_mass:f32)->f32{
  let mars_weight = (earth_mass / 9.81) * 3.711;
  return mars_weight;
}