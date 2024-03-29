use crate::db::users::User;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexParams {
    username: Option<String>,
}

pub async fn index( req: HttpRequest) -> impl Responder {
    let username = req.extensions().get::<User>().map(|u| u.username.clone());
    HttpResponse::Ok().body(IndexParams { username }.render().unwrap())
}
