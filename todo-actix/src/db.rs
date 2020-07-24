use crate::models::{TodoList, TodoItem};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client:&Client)-> Result<Vec<TodoList>,io::Error> {
  // get all items from todo list
  let statement = client.prepare("select * from todo_list;").await.unwrap();

  let todos = client.query(&statement,&[])
    .await
    .expect("Error getting todo list")
    .iter()
    .map(|row| TodoList::from_row_ref(row).unwrap())
    .collect::<Vec<TodoList>>();

  //return todos vector
  Ok(todos)
}


pub async fn get_items(client:&Client, list_id:i32)-> Result<Vec<TodoItem>,io::Error> {
  // get all items from todo list
  let statement = client.prepare("select * from todo_item where list_id = $1 order by id;").await.unwrap();

  //here we pass query and variables to be added
  let items = client.query(&statement,&[&list_id])
    .await
    .expect("Error getting todo items")
    .iter()
    .map(|row| TodoItem::from_row_ref(row).unwrap())
    .collect::<Vec<TodoItem>>();

  //return items vector
  Ok(items)
}

pub async fn create_todo_list(client: &Client, title: String) -> Result<TodoList, io::Error> {

  // get all items from todo list
  let statement = client.prepare("insert into todo_list (title) values($1) returning id,title;").await.unwrap();

  //here we pass query and variables to be added
  client.query(&statement,&[&title])
    .await
    .expect("Error crating todo list")
    .iter()
    .map(|row| TodoList::from_row_ref(row).unwrap())
    .collect::<Vec<TodoList>>()
    .pop()
    .ok_or(io::Error::new(io::ErrorKind::Other,"Error creating todo list"))

}