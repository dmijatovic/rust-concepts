use deadpool_postgres::{Pool, PoolError};
use tokio_postgres::row::Row;
use tokio_postgres::types::ToSql;

use crate::graphql::TodoList;

// map return from tokio:postreges into stuct
impl From<&Row> for TodoList {
  fn from(row: &Row) -> Self {
    Self {
      id: row.get("id"),
      title: row.get("title"),
    }
  }
}

pub async fn get_todos(pool: &Pool) -> Result<Vec<TodoList>, PoolError> {
  // create raw sql statement
  let raw_sql = "SELECT * FROM todo_list;";
  // execute query or propagate error
  let rows = todo_query(pool, raw_sql, &[]).await?;
  // return rows
  Ok(rows)
}

pub async fn get_todo_list(pool: &Pool, uid: i32) -> Result<Vec<TodoList>, PoolError> {
  //sql to run
  let raw_sql = "SELECT * FROM todo_list WHERE id=$1;";
  //execute query with params
  let row = todo_query(pool, raw_sql, &[&uid]).await?;
  //return row
  Ok(row)
}

pub async fn add_todo_list(pool: &Pool, title: &str) -> Result<Vec<TodoList>, PoolError> {
  //sql to run
  let raw_sql = "INSERT INTO todo_list (title) VALUES($1) RETURNING id,title;";
  //execute query with params
  let row = todo_query(pool, raw_sql, &[&title]).await?;
  //return row
  Ok(row)
}

pub async fn update_todo_list(
  pool: &Pool,
  id: i32,
  title: &str,
) -> Result<Vec<TodoList>, PoolError> {
  //sql to run
  let raw_sql = "UPDATE todo_list SET title=$1 WHERE id=$2 RETURNING id,title;";
  //execute query with params
  let row = todo_query(pool, raw_sql, &[&title, &id]).await?;
  //return row
  Ok(row)
}

pub async fn delete_todo_list(pool: &Pool, id: i32) -> Result<Vec<TodoList>, PoolError> {
  //sql to run
  let raw_sql = "DELETE from todo_list WHERE id=$1 RETURNING id,title;";
  //execute query with params
  let row = todo_query(pool, raw_sql, &[&id]).await?;
  //return row
  if row.len() == 0 {
    println!("Nothing DELETED!!! Check id");
  }
  Ok(row)
}

// Runs all tokio_postgres queries for todos. Provide pool, raw sql string and the list of parameters to be used in raw_sql. If no parameters provide refference to empty list/array like this &[].
async fn todo_query(
  pool: &Pool,
  raw_sql: &str,
  args: &'_ [&'_ (dyn ToSql + Sync)],
) -> Result<Vec<TodoList>, PoolError> {
  // get connection from pool
  let cnn = pool.get().await?;

  // prepare returns tokio_postgres::Error
  // `RunError` impl `From<std::error::Error>`
  let sql = cnn.prepare(raw_sql).await?;

  // execute query and collect results
  let rows = cnn
    .query(&sql, args)
    .await?
    .iter()
    .map(|row: &Row| TodoList::from(row))
    .collect::<Vec<TodoList>>();

  Ok(rows)
}
