use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
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
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::ACCEPT,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(transpile)
    })
    .bind(("127.0.0.1", 8080))?;

    println!("Servidor iniciado en http://127.0.0.1:8080");
    server.run().await
}