#[macro_use] 
extern crate diesel;
extern crate serde_derive;

use actix_web::{middleware::Logger, web, App, HttpServer, HttpRequest, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use env_logger::Env;

use actix_web_static_files::{self, ResourceFiles};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod handlers;
mod schema;
mod models;
mod libs;



#[actix_web::main]
async fn main() -> std::io::Result<()>{

    //load .env into environment
    dotenv::dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    //setup database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create db pool.");


    HttpServer::new(move ||{

        let generated = generate();
            
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::show)
            .service(handlers::build)
            .service(handlers::srch)
            .service(ResourceFiles::new("/", generated))
            
    })
        //.bind(("127.0.0.1", 8080))?
        .bind(("192.168.1.6", 8080))?
        .run()
        .await
}

