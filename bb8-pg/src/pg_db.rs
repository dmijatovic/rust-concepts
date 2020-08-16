#![allow(dead_code)]
use bb8::{Pool,RunError};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{NoTls,Error, Config, Client};
use tokio_postgres::row::{Row};
use std::time::Duration;

use crate::todo;

fn new_config()->Config{
  let mut pg_cfg = tokio_postgres::config::Config::new();
  pg_cfg.host("localhost");
  pg_cfg.port(5432);
  pg_cfg.user("postgres");
  pg_cfg.password("changeme");
  pg_cfg.dbname("todo_db");
  pg_cfg.connect_timeout(Duration::new(5,0));
  return pg_cfg;
}

async fn get_todo_items(c:&Client)->Result<Vec<Row>,Error>{
  c.query("SELECT * FROM todo_item",&[]).await
}

async fn get_users(c:&Client)->Result<Vec<Row>,Error>{
  c.query("SELECT * FROM users",&[]).await
}

async fn tokio_example(){
  //create tokio configuration
  let pg_cfg = new_config();
  // connect to db?
  let (client, cnn) = match pg_cfg.connect(NoTls)
    .await{
      Ok(resp)=>{resp},
      Err(e)=>panic!("tokio_example error: {:?}", e)
    };

  // The connection object performs the actual communication with the database,
  // so spawn it off to run on its own.
  tokio::spawn(async move {
    if let Err(e) = cnn.await {
        eprintln!("connection error: {}", e);
    }
  });

  println!("pg_cfg: {:?}", pg_cfg);

  match get_users(&client).await{
    Ok(rows)=> println!("rows: {:?}", rows),
    Err(e)=>eprintln!("tokio_example error: {:?}", e)
  };
}


async fn pool_query(
  pool: &Pool<PostgresConnectionManager<tokio_postgres::NoTls>>,
) -> Result<Vec<Row>, RunError<tokio_postgres::Error>> {
  // get connection from pool
  let cnn = pool.get().await?;

  // although prepare and query_one return tokio_postgres::Error
  // `RunError` impl `From<std::error::Error>`
  let sql = cnn.prepare("SELECT * FROM users;").await?;

  // let row = cnn.query_one(&stmt, &[]).await?;
  let rows = cnn.query(&sql,&[]).await?;
  // &sql,&[]).await?;
  Ok(rows)
}

async fn get_todo_lists(pool: &Pool<PostgresConnectionManager<tokio_postgres::NoTls>>,
) -> Result<Vec<Row>, RunError<tokio_postgres::Error>> {
  // get connection from pool
  let cnn = pool.get().await?;

  // although prepare and query_one return tokio_postgres::Error
  // `RunError` impl `From<std::error::Error>`
  let sql = cnn.prepare("SELECT * FROM todo_list;").await?;

  // let row = cnn.query_one(&stmt, &[]).await?;
  let rows = cnn.query(&sql,&[]).await?;
  // &sql,&[]).await?;
  Ok(rows)
}

async fn create_pool()->Result<Pool<PostgresConnectionManager<NoTls>>,Error>{
  //create tokio configuration
  let pg_cfg = new_config();
  let pg_mgr = PostgresConnectionManager::new(pg_cfg, NoTls);

  // pg_mgr.
  // build a pool (default is 10 connections in the pool)
  match Pool::builder().build(pg_mgr).await {
    Ok(pool) => Ok(pool),
    Err(e) => panic!("create_pool error: {:?}", e),
  }
}



async fn bb8_example(){
  //create tokio configuration
  // let pg_cfg = new_config();
  // let pg_mgr = PostgresConnectionManager::new(pg_cfg, NoTls);

  // pg_mgr.
  // build a pool (default is 10 connections in the pool)
  // let pool = match Pool::builder().build(pg_mgr).await {
  //   Ok(pool) => pool,
  //   Err(e) => panic!("builder error: {:?}", e),
  // };

  let pool = match create_pool().await{
    Ok(pool)=>pool,
    Err(e)=> panic!("Error occurred: {}", e),
  };

  // match pool_query(&pool).await {
  //   Ok(result) => println!("result: {:?}", result),
  //   Err(error) => eprintln!("Error occurred: {}", error),
  // }

  // get list of todos
  match todo::get_todos(&pool).await {
    Ok(result) => println!("todo_lists: {:?}", result),
    Err(error) => panic!("get_todos error: {}", error),
  }
  //get specific todo list by id
  let id:i32 = 1;
  match todo::get_todo_list(&pool,id).await {
    Ok(result) => println!("get_todo_list: {:?}", result),
    Err(error) => panic!("get_todo_list error: {}", error),
  }

  //add todo list
  let title:&str = "This is my title";
  match todo::add_todo_list(&pool,title).await {
    Ok(result) => println!("add_todo_list: {:?}", result),
    Err(error) => panic!("add_todo_lists error: {}", error),
  }

  //update todo list
  let title:&str = "This is updated title";
  match todo::update_todo_list(&pool,1,title).await {
    Ok(result) => println!("update_list_item: {:?}", result),
    Err(error) => panic!("update_todo_lists error: {}", error),
  }

  //delete todo list
  match todo::delete_todo_list(&pool,7).await {
    Ok(result) => println!("delete_list_item: {:?}", result),
    Err(error) => panic!("delete_todo_lists error: {}", error),
  }

}




#[tokio::main]
pub async fn main ()->Result<(),Error>{
  // let cnn_str = String::from("postgresql://postgres:changeme@localhost:5432/todo_db");

  // let (client,cnn) = tokio_postgres::connect(&cfg, NoTls).await?;
  // tokio_example().await;
  bb8_example().await;

  Ok(())
}

// pub fn main(){

// }