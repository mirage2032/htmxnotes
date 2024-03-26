use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use askama::Template;
use crate::middleware::auth::SessionCookie;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexParams;

pub async fn index(req: HttpRequest) -> impl Responder {
    let extensions = req.extensions();
    if let Some(cookie) = extensions.get::<SessionCookie>() {
        println!("Token: {}", cookie.token);
    }
    HttpResponse::Ok().body(IndexParams.render().unwrap())
}
