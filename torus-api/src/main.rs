use actix_web::{App, HttpServer};
use torus_api::handlers::sandbox_handler::{
    create_sandbox, delete_sandbox_by_id, get_sandbox, get_sandbox_by_id, update_sandbox,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_sandbox)
            .service(get_sandbox_by_id)
            .service(create_sandbox)
            .service(update_sandbox)
            .service(delete_sandbox_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
