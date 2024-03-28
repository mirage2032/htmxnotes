use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use askama::Template;
use crate::db::users::User;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexParams{
    username: Option<String>,
}

pub async fn index(req: HttpRequest) -> impl Responder {
    let username = req.extensions().get::<User>().map(|u| u.username.clone());
    HttpResponse::Ok().body(IndexParams{username}.render().unwrap())
}
