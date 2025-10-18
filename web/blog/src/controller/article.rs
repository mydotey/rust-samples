use actix_web::{HttpResponse, Responder, get, post, web::Json};
use blog_client::content::CreateArticleDto;
use log::error;

use crate::{
    controller::{handle_request, map},
    domain::content::Article,
};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[post("/api/entity/article/create")]
pub async fn create_article(json: Json<CreateArticleDto>) -> impl Responder {
    match handle_request(json, &crate::service::article::create_article) {
        Ok(model) => HttpResponse::Created().json(model),
        Err(e) => {
            error!("Failed to create article: {}", e);
            HttpResponse::InternalServerError().body("Failed to create article")
        }
    }
}
