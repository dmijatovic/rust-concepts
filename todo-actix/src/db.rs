use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::models::{TodoList, TodoItem, CreateTodoItem};
use crate::errors::{AppError, AppErrorType};

pub async fn get_todo_lists(client:&Client)-> Result<Vec<TodoList>,AppError> {
  // get all items from todo list
  let statement = client
    .prepare("select * from todo_list;")
    .await
    .map_err(|err| AppError{
      message: None,
      //wrap in Some because it is optinal value
      cause: Some(err.to_string()),
      error_type: AppErrorType::DbError
    })?; //question mark at the end optinally returns it

  let todos = client.query(&statement,&[])
    .await
    .expect("Error getting todo list")
    .iter()
    .map(|row| TodoList::from_row_ref(row).unwrap())
    .collect::<Vec<TodoList>>();

  //return todos vector
  Ok(todos)
}


pub async fn get_list_items(client:&Client, list_id:i32)-> Result<Vec<TodoItem>,io::Error> {
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


pub async fn create_todo_item(client: &Client, todo_item: &CreateTodoItem) -> Result<TodoItem, io::Error> {

  // get all items from todo list
  let statement = client.prepare("insert into todo_item (title,checked,list_id) values($1,$2,$3) returning id,title,checked,list_id;").await.unwrap();

  //here we pass query and variables to be added
  client.query(&statement,&[&todo_item.item.title,&todo_item.item.checked,&todo_item.list_id])
    .await
    .expect("Error crating todo list")
    .iter()
    .map(|row| TodoItem::from_row_ref(row).unwrap())
    .collect::<Vec<TodoItem>>()
    .pop()
    .ok_or(io::Error::new(io::ErrorKind::Other,"Error creating todo item"))

}


pub async fn check_todo_item(client: &Client, list_id: u32, item_id: u32) -> Result<(),io::Error>{
  let statement = client.prepare("update todo_items set checked = true where list_id = $1 and id = $2 and checked = false").await.unwrap();

  let result = client.execute(&statement,&[&list_id,&item_id])
    .await
    .expect("Error checking todo item");

  match result{
    ref updated if *updated == 1 => Ok(()),
    _ => Err(io::Error::new(io::ErrorKind::Other,"Failed to check todo item"))
  }
}