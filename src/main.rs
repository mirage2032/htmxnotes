use actix_files as fs;
use actix_web::{App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::all_routes)
            .service(fs::Files::new("/static", "static"))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
