
use std::thread;
use std::time;



fn thread1()->bool{
  for i in 1..10{
    println!("Doing some work in thread1: {}", i);
    thread::sleep(time::Duration::from_millis(500));
  }
  return true;
}

fn thread2(){
  for i in 1..10{
    println!("Doing some work in thread2: {}", i);
    thread::sleep(time::Duration::from_millis(100));
  }
}

fn main() {
  println!("Concurrency starting");
  let handle1 = thread::spawn(thread1);
  // thread1();
  let handle2 = thread::spawn(|| {
    thread2();
  });
  println!("Starting threads");
  // start thread 1
  let result1 = handle1.join();
  println!("Thread 1 result {:?}", result1);
  // start thread 2 - note that should finish earlier 
  // but it does not print the line untill thread 1
  // is done
  let result2 = handle2.join();
  println!("Thread 2 result {:?}", result2);
}
