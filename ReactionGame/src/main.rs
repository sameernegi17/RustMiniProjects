use std::sync::Mutex;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};

use actix_files::NamedFile;

use crate::leaderboard::{Leaderboard, Record};

mod leaderboard;



struct AppState
{
    leaderboard: Mutex<Leaderboard>
}

async fn index() -> Result<NamedFile>
{
    Ok(NamedFile::open("static/index.html")?)
}


async fn submit_scores(data: web::Data<AppState>, form: web::Form<Record>) -> Result<NamedFile>
{
    let mut lb = data.leaderboard.lock().unwrap();
    lb.add_record(form.into_inner());
    Ok(NamedFile::open("static/index.html")?)
}

async fn get_leaderboard(data: web::Data<AppState>) -> impl Responder
{
    let lb = data.leaderboard.lock().unwrap();
    HttpResponse::Ok().json(lb.get_top(10))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    println!("Server Running at 127.0.0.1:8080");
    let leaderboard = web::Data::new(AppState {
        leaderboard: Mutex::new(Leaderboard::new()),
    });

    HttpServer::new(move || {
        App::new()
        .app_data(leaderboard.clone())
        .service(actix_files::Files::new("/static", "static").show_files_listing())
        .route("/", web::get().to(index))
        .route("/submit", web::post().to(submit_scores))
        .route("/leaderboard", web::get().to(get_leaderboard))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}