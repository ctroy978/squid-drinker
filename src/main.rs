#[macro_use]
extern crate diesel;

use actix_web::{get, web, App, HttpServer, Responder};
use actix_cors::Cors;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod handlers;
mod schema;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    //load .env into environment
    dotenv::dotenv().ok();

    //setup database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create db pool.");


    HttpServer::new(move ||{

        //allows crosstalk remove later.
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"]);
            
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(handlers::show)
    })
        //.bind(("127.0.0.1", 8080))?
        .bind(("192.168.1.113", 8080))?
        .run()
        .await
}

