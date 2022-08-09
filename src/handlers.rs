use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/drink/{title}")]
async fn show(title: web::Path<String>) -> impl Responder{

    


    HttpResponse::Ok().body(format!("Let us make a {}", title))
}

