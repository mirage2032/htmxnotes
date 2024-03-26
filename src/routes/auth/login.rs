use actix_web::{get, HttpRequest, HttpResponse, post, Responder, cookie::Cookie, web};
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LoginParams;

#[derive(Serialize, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[get("/login")]
pub async fn login_get(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body(LoginParams.render().unwrap())
}

#[post("/login")]
pub async fn login_post(form: web::Form<LoginForm>) -> impl Responder {
    HttpResponse::Ok()
        .cookie(Cookie::build("SESSION", form.username.clone()).path("/").finish())
        .insert_header(("HX-Redirect", "/"))
        .body("Success!")
}
