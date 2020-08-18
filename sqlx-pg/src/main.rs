
use sqlx::postgres::PgPool;

mod pg_db{
  pub mod main;
  pub mod cnn;
  pub mod todo_list;
}

#[tokio::main]
async fn main() {
  println!("Hello, world!");
  // pg_db::main::create_pool();
  let cnn_str="postgres://postgres:changeme@localhost:5432/todo_db".to_string();
  let pool:PgPool = match pg_db::cnn::create_pool(&cnn_str)
    .await{
      Ok(pool)=>pool,
      Err(e)=>panic!("main error:{:?}",e)
    };

  println!("pool: {:?}", pool);
}
