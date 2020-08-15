#![allow(dead_code)]
use bb8::{Pool,RunError};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{NoTls,Error, Config, Client};
use tokio_postgres::row::{Row};

fn new_config()->Config{
  let mut pg_cfg = tokio_postgres::config::Config::new();
  pg_cfg.host("localhost");
  pg_cfg.port(5432);
  pg_cfg.user("postgres");
  pg_cfg.password("changeme");
  pg_cfg.dbname("todo_db");
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
  let cnn = pool.get().await.unwrap();

  // although prepare and query_one return tokio_postgres::Error
  // `RunError` impl `From<std::error::Error>`
  let sql = cnn.prepare("SELECT * FROM users;").await?;

  // let row = cnn.query_one(&stmt, &[]).await?;
  let rows = cnn.query(&sql,&[]).await?;
  // &sql,&[]).await?;
  Ok(rows)
}

async fn bb8_example(){
  //create tokio configuration
  let pg_cfg = new_config();
  let pg_mgr = PostgresConnectionManager::new(pg_cfg, NoTls);

  // pg_mgr.
  // build a pool (default is 10 connections in the pool)
  let pool = match Pool::builder().build(pg_mgr).await {
    Ok(pool) => pool,
    Err(e) => panic!("builder error: {:?}", e),
  };

  match pool_query(&pool).await {
    Ok(result) => println!("result: {:?}", result),
    Err(error) => eprintln!("Error occurred: {}", error),
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