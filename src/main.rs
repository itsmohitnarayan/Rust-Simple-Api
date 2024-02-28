use std::path;

use actix_web::{self, dev::Path, get, App, HttpServer, Responder};{
    HttpServer,
    get,
    App,
    web::Path,
    Responder,
}

use rhai::Engine;

#[get("multiply/{num1}/{num2}")]
async fn multiply(Path<(i64, i64)>) -> impl Responder{
    let (num1, num2) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2)
}

#[actix_web::main]
async fn main () -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
        .services(multiply)
        .services(add)
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
}