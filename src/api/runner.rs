use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use crate::core::core::transpile_content;

#[post("/transpile")]
async fn transpile(req_body: String) -> impl Responder {
    match transpile_content(&req_body) {
        Ok(output) => HttpResponse::Ok().body(output),
        Err(err) => {
            eprintln!("Transpiling error: {}", err);
            HttpResponse::InternalServerError().body("There was an error porcessing the input")
        }
    }
}

#[actix_web::main]
pub(crate) async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new().service(transpile)
    })
        .bind(("127.0.0.1", 8080))?;

    println!("Servidor iniciado en http://127.0.0.1:8080");
    server.run().await
}