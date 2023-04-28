use std::env;
use salvo_extra::affix;
use salvo::prelude::*;
use sea_orm::{Database, DatabaseConnection};
use controllers::book_controller;
use book_controller::AppState;

mod entities;
mod daos;
mod services;
mod controllers;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(&db_url).await.unwrap();
    let state = AppState { conn };

    let router = Router::new()
        .hoop(affix::inject(state))
        .get(book_controller::all_books)
        .push(Router::with_path("add").post(book_controller::add_book))
        .push(Router::with_path("update").post(book_controller::update_book))
        .push(Router::with_path("delete/<id>").post(book_controller::delete_book));

    Server::new(TcpListener::bind(&server_url))
        .serve(router)
        .await;
}