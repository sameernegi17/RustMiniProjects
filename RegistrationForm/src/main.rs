
use std::fs;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct RegistrationForm
{
    name : String,
    email : String,
    password : String,
}

async  fn show_me() -> impl Responder
{
    let html = fs::read_to_string("templates/register.html").unwrap_or_else(|_| "<H1>  Error Loading</h1>".to_string());

    HttpResponse::Ok().content_type("text/html").body(html)
}


async fn register(form: web::Form<RegistrationForm>) -> impl Responder
{
    println!("Recieved Registration: {:?}", form);

    let response = format!("<h2> Thanks for registering <h2>");

     HttpResponse::Ok().content_type("text/html").body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");
    HttpServer::new(|| {

        App::new()
        .route("/", web::get().to(show_me))
        .route("/register", web::post().to(register))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
