use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Template, Serialize, Deserialize)]
#[template(path = "index.html")]
pub struct IndexParams {
    name: String,
}

pub async fn index(info: web::Query<IndexParams>) -> impl Responder {
    HttpResponse::Ok().body(info.render().unwrap())
}
