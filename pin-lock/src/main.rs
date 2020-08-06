use std::io::stdin;

enum LockState{
  Locked,
  Open,
  Failed,
}

fn main() {
  let code = String::from("1234");
  let mut state = LockState::Locked;
  let mut entry = String::new();

  loop{
    match state{
      LockState::Locked => {
        let mut input = String::new();
        match stdin().read_line(&mut input){
          Ok(_)=>{
            entry.push_str(&input.trim_end())
          },
          Err(_)=> continue
        }
        if entry == code {
          state = LockState::Open;
          continue
        }
        if !code.starts_with(&entry){
          state = LockState::Failed;
        }
      },
      LockState::Failed =>{
        println!("Code incorect");
        entry.clear();
        state = LockState::Locked;
      },
      LockState::Open =>{
        println!("You got it!");
        return
      }
    }
  }
}
