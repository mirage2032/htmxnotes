use actix_web::web;
use actix_web_lab::middleware::from_fn;
use crate::middleware::auth::authenticate;

mod auth;
mod index;

pub fn all_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index::index)).wrap(from_fn(authenticate)));
    cfg.service(web::scope("/auth").configure(auth::auth_routes));
}
