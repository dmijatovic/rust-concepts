// use sqlx::Pool;
use sqlx::postgres::PgPool;
// use std::error::Error;
use std::time::Duration;

pub async fn create_pool(cnn_str: &str)->Result<PgPool,String>{
  // create connection pool
  let pool = PgPool::builder()
    .max_size(20)
    .connect_timeout(Duration::from_secs(5))
    .build(cnn_str)
    .await;

  // return results
  match pool{
    Ok(pool)=>Ok(pool),
    Err(e)=>panic!("create_pool error: {:?}", e)
  }
}