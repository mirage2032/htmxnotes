use actix_files as fs;
use actix_web::{App, HttpServer, web};
use actix_web_lab::middleware::from_fn;
use actix_htmx::HtmxMiddleware;

mod routes;
mod middleware;
mod db;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address = "127.0.0.1:8000";
    println!("Starting server at: {}", bind_address);
    HttpServer::new(|| {
        App::new()
            .wrap(HtmxMiddleware)
            .app_data(web::Data::new(db::create_pool()))
            .configure(routes::all_routes)
            .wrap(from_fn(middleware::nocache::nocache))
            .service(fs::Files::new("/static", "static"))
    })
        .bind(bind_address)?
        .run()
        .await
}
