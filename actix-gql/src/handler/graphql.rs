use actix_web::{get, post, web, Error, HttpResponse};
use deadpool_postgres::Pool;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

use crate::schema::{Context, Schema};

#[get("/graphql")]
pub async fn get_graphql() -> HttpResponse {
  HttpResponse::Ok()
    .content_type("text/html")
    .body("<h1>GraphQL main point GET</h1>")
}

#[post("/graphql")]
pub async fn post_graphql(
  pool: web::Data<Pool>,
  schema: web::Data<Arc<Schema>>,
  data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
  // get pool reference
  // and pass it to context
  let ctx = Context {
    // get reference to inner object Pool
    db: pool.get_ref().to_owned(),
  };
  // execute query and serialize response
  // let res = data.execute_async(&schema, &ctx).await;
  let res = web::block(move || {
    let res = data.execute(&schema, &ctx);
    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
  })
  .await
  .map_err(Error::from)?;

  // return OK result
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(res),
  )
}
