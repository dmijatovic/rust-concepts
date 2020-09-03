use deadpool_postgres::Manager;
use std::time::Duration;
use tokio_postgres::{Config, NoTls};

pub mod todo_item;
pub mod todo_list;

fn new_config() -> Config {
  let mut pg_cfg = tokio_postgres::config::Config::new();
  pg_cfg.host("localhost");
  pg_cfg.port(5432);
  pg_cfg.user("postgres");
  pg_cfg.password("changeme");
  pg_cfg.dbname("todo_db");
  pg_cfg.connect_timeout(Duration::new(5, 0));
  return pg_cfg;
}

pub type Pool = deadpool_postgres::Pool;

pub async fn create_pool() -> Pool {
  //create configutation
  let cfg = new_config();
  //create pool manager
  let mgr = Manager::new(cfg, NoTls);
  //create new pool
  Pool::new(mgr, 16)
}

// make queries avaliable to crate
// pub use todo_item;
// pub use todo_list;
