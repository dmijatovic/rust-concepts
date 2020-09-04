use actix_web::{get, web::ServiceConfig, HttpResponse};

use crate::graphql as gql;

mod graphql;
use graphql::{get_graphql, post_graphql};

#[get("/")]
pub async fn index() -> HttpResponse {
  HttpResponse::Ok()
    .content_type("text/html")
    .body("<h1>Actix GraphQL example</h1>")
}

pub fn register(cfg: &mut ServiceConfig) {
  // Create Juniper schema
  let schema = std::sync::Arc::new(gql::create_schema());
  //add graphQL schema and routes
  cfg
    .data(schema)
    // shows graphiql user interace
    .service(get_graphql)
    // end point for graphql queries
    .service(post_graphql)
    .service(index);
}
