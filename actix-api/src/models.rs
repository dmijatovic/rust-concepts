use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct BaseTodoList{
  pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList{
  pub id: u64,
  pub title: String,
}

#[derive(Serialize, Debug)]
pub struct TodoItem{
  pub id: u64,
  pub list_id: u64,
  pub title: String,
  pub checked: bool,
}

