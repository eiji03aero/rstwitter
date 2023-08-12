use actix_web::{web, App, HttpServer};
use sea_orm::{ConnectOptions, Database};
use std::env;

use rstwitter::handlers;

const HOST: &str = "0.0.0.0";
const PORT: u16 = 8090;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let opt = ConnectOptions::new(database_url.to_owned());

        let db = Database::connect(opt).await.unwrap();

        handlers::AppState { db }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(handlers::users::list)
            .service(handlers::users::create)
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
