use actix_web::{get, HttpResponse};

use juniper::http::graphiql::graphiql_source;

#[get("/graphiql")]
pub async fn get_graphiql() -> HttpResponse {
  let html = graphiql_source("http://127.0.0.1:8080/graphql");
  HttpResponse::Ok().content_type("text/html").body(html)
}
