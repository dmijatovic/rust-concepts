

// basic generic using 2 different types
#[derive(Debug)]
struct Point<T,S>{
  x: T,
  y: T,
  active: S
}

#[derive(Debug)]
struct Line<T,S>{
  start: Point<T,S>,
  end: Point<T,S>,
}

pub fn main(){
  println!("Generics");

  let p = Point{
    x:10,
    y:34,
    active: false
  };

  let p2 = Point{
    x:true,
    y:false,
    active: "Yes"
  };

  println!("{:?}",p);

  println!("{:?}",p2);

}