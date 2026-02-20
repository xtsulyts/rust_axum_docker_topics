<<<<<<< HEAD
fn main() {
    println!("Hello, world!");
}
=======
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
>>>>>>> 501a1e3 (dependencias, actix, serde, tokio)
