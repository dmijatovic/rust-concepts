#![allow(dead_code)]

use bb8::{Pool,RunError};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::row::{Row};
use tokio_postgres::types::{ToSql};
// use std::convert::From;

// use serde::{Serialize};

#[derive(Debug)]
pub struct NewTodoItem{
  pub list_id: i32,
  pub title: String,
  pub checked: bool,
}

#[derive(Debug)]
pub struct TodoItem{
  pub id:i32,
  pub list_id: i32,
  pub title: String,
  pub checked: bool,
}
// map return from tokio:postreges into stuct
impl From<&Row> for TodoItem{
  fn from(row: &Row) -> Self {
    Self {
      id: row.get("id"),
      list_id: row.get("list_id"),
      title: row.get("title"),
      checked: row.get("checked"),
    }
  }
}


fn get_todo_item_from_row(row: &Row)->TodoItem{
  TodoItem::from(row)
}


// Runs all tokio_postgres queries for todo items. Provide bb8-pool, raw sql string and the list of parameters to be used in raw_sql. If no parameters provide refference to empty list/array like this &[].
async fn todo_item_query(pool: &Pool<PostgresConnectionManager<tokio_postgres::NoTls>>,raw_sql:&str,args:&'_ [&'_ (dyn ToSql + Sync)])->Result<Vec<TodoItem>, RunError<tokio_postgres::Error>>{
  // get connection from pool
  let cnn = pool.get().await?;

  // prepare returns tokio_postgres::Error
  // `RunError` impl `From<std::error::Error>`
  let sql = cnn.prepare(raw_sql).await?;

  // execute query and collect results
  let rows = cnn.query(&sql,args)
    .await?
    .iter()
    .map(get_todo_item_from_row)
    .collect::<Vec<TodoItem>>();

  Ok(rows)
}

pub async fn get_todo_items(pool: &Pool<PostgresConnectionManager<tokio_postgres::NoTls>>,
list_id:i32) -> Result<Vec<TodoItem>, RunError<tokio_postgres::Error>> {
  // get connection from pool
  // let cnn = pool.get().await?;

  // although prepare and query_one return tokio_postgres::Error
  // `RunError` impl `From<std::error::Error>`
  let raw_sql = "SELECT * FROM todo_item WHERE list_id=$1;";

  // let row = cnn.query_one(&stmt, &[]).await?;
  let rows = todo_item_query(pool, raw_sql, &[&list_id]).await?;
  // return rowns
  Ok(rows)
}


pub async fn get_todo_item(pool: &Pool<PostgresConnectionManager<tokio_postgres::NoTls>>,
uid:i32) -> Result<Vec<TodoItem>, RunError<tokio_postgres::Error>> {
  //sql to run
  let raw_sql = "SELECT * FROM todo_item WHERE id=$1;";
  //execute query with params
  let row = todo_item_query(pool, raw_sql,&[&uid]).await?;
  //return row
  Ok(row)
}

pub async fn add_todo_item(pool: &Pool<PostgresConnectionManager<tokio_postgres::NoTls>>,item:&NewTodoItem) -> Result<Vec<TodoItem>, RunError<tokio_postgres::Error>> {
    //sql to run
  let raw_sql = "INSERT INTO todo_item (list_id,title,checked) VALUES($1,$2,$3) RETURNING id,list_id,title,checked;";
  //execute query with params
  let row = todo_item_query(pool, raw_sql,&[&item.list_id,&item.title,&item.checked]).await?;
  //return row
  Ok(row)
}

pub async fn update_todo_item(pool: &Pool<PostgresConnectionManager<tokio_postgres::NoTls>>,
  item:&TodoItem) -> Result<Vec<TodoItem>, RunError<tokio_postgres::Error>> {
    //sql to run
  let raw_sql = "UPDATE todo_item SET list_id=$1, title=$2, checked=$3 WHERE id=$4 RETURNING id,list_id,title,checked;";
  //execute query with params
  let row = todo_item_query(pool, raw_sql,&[&item.list_id,&item.title,&item.checked,&item.id]).await?;
  //return row
  Ok(row)
}

pub async fn delete_todo_item(pool: &Pool<PostgresConnectionManager<tokio_postgres::NoTls>>,
  id:i32) -> Result<Vec<TodoItem>, RunError<tokio_postgres::Error>> {
    //sql to run
  let raw_sql = "DELETE from todo_item WHERE id=$1 RETURNING id,list_id,title,checked;";
  //execute query with params
  let row = todo_item_query(pool, raw_sql,&[&id]).await?;
  //return row
  if row.len()==0{
    println!("Nothing DELETED!!! Check id");
  }
  Ok(row)
}