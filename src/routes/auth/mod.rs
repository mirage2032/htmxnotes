use actix_web::web;

mod login;
mod register;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").get(login::login));
    cfg.service(web::resource("/register").get(register::register));
}
