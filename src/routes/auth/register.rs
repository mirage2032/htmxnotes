use crate::db::users::{NewUser, User};
use crate::db::DBPool;
use crate::routes::auth::AuthTemplate;
use actix_htmx::Htmx;
use actix_web::{cookie::Cookie, get, post, web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

#[get("/register")]
pub async fn register_get(_: HttpRequest) -> impl Responder {
    let mut page = AuthTemplate::new("/auth/register", "Register");
    page.add_element("Username", "text", Some("username"));
    page.add_element("Email", "email", Some("email"));
    page.add_element("Password", "password", Some("password"));
    page.add_element("Verify password:", "password", None);
    HttpResponse::Ok().body(page.render().unwrap())
}

#[post("/register")]
pub async fn register_post(
    dbpool: web::Data<DBPool>,
    htmx: Htmx,
    form: web::Form<RegisterForm>,
) -> impl Responder {
    let conn = dbpool.as_ref();
    let newuser = match NewUser::new(&form.username, &form.email, &form.password) {
        Ok(user) => user,
        Err(err) => {
            return HttpResponse::BadRequest().body(format!("Invalid form data!: {}",err));
        }
    };
    match User::create(conn, newuser) {
        Ok(user) => {
            match user.create_session(conn, Duration::from_secs(60 * 60 * 24 * 2)) {
                Ok(token) => {
                    htmx.redirect("/".to_string());
                    HttpResponse::Ok()
                        .cookie(Cookie::build("SESSION", token).path("/").finish())
                        .body("Success!")
                }
                Err(err) => {
                    //if session could not be generated redirect to login page
                    htmx.redirect("/auth/login".to_string());
                    HttpResponse::Ok().body(format!(
                        "Success! Could not authenticate automatically.\n{}",
                        err
                    ))
                }
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error!: {}", err)),
    }
}
