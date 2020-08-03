use std::fs::File;

pub fn panic_now(msg:String){
  panic!(msg)
}

// pub fn custom_error(return_err:bool){
//   if return_err==true{
//     panic!("You wanted an error!")
//   }else{
//     Ok("I send you a success!")
//   }
// }


pub fn file_open_error(){
  let f=File::open("no_that_file.txt");
  // file open returns io::Result
  match f{
    Ok(_)=>println!("File found"),
    Err(err)=>println!("Error: {:?}", err)
  }
}

pub fn file_open_error2(){
  File::open("no_that_file.txt")
    .expect("File not found");

  println!("File found")
}