

fn basic_borrow(x:&Vec<i32>){
  println!("Basic borrow {:?}", x)
}


fn mutable_borrow(x: &mut Vec<i32>){
  //change first element to 101
  x[0]=101
}

pub fn main(){
  let x = vec![3,2,1];
  let mut y = vec![3,2,1];
  // pass reference to x vec
  basic_borrow(&x);
  // we can use it after borrowing
  println!("X from main {:?}", x);
  println!("Y from main {:?}", y);
  // pass mutatble reference
  mutable_borrow(&mut y);
  println!("Y from main {:?} after mut borrow", y);
}