use actix_web::{get, web::ServiceConfig, HttpResponse};

use crate::schema;

mod graphiql;
mod graphql;

pub use graphiql::get_graphiql;
pub use graphql::get_graphql;
pub use graphql::post_graphql;

#[get("/")]
pub async fn index() -> HttpResponse {
  HttpResponse::Ok()
    .content_type("text/html")
    .body("<h1>Actix GraphQL example</h1>")
}

pub fn register(cfg: &mut ServiceConfig) {
  // Create Juniper schema
  let schema = std::sync::Arc::new(schema::create_schema());
  //add graphQL schema and routes
  cfg
    .data(schema)
    .service(get_graphiql)
    .service(get_graphql)
    .service(post_graphql)
    .service(index);
}
