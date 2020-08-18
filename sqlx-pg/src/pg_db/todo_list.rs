use sqlx::postgres::{PgPool,PgRow};
#[allow(unused_imports)]
use sqlx::postgres::PgQueryAs;
use sqlx::Row;
// use sqlx::QueryAs;
// use std::error::Error;
// use sqlx::postgres::;

#[derive(Debug, sqlx::FromRow)]
pub struct TodoList{
  id:i32,
  title: String
}

// impl From<&Row> for TodoList{
//   fn from(row: &Row) -> Self {
//     Self {
//       id: row.get("id"),
//       title: row.get("title"),
//     }
//   }
// }

// fn get_todo_list_from_row(row: &Row)->TodoList{
//   TodoList::from(row)
// }

// impl<'q, O> PgQueryAs<'q, O> for QueryAs<'q, Postgres, O>{}
// pub async fn get_todos(pool: &PgPool) -> Result<Vec<TodoList>, String> {
pub async fn get_todos(pool: &PgPool){
  // `RunError` impl `From<std::error::Error>`
  let raw_sql = "SELECT * FROM todo_list;";

  let rows = sqlx::postgres::PgQueryAs


    .fetch_all(pool)
    .await;
    // .iter()
    // .collect::<Vec<TodoList>>();

  println!("get_todos: {:?}",rows);
  // Ok(rows)
}
