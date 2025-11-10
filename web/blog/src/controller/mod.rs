use std::panic::{RefUnwindSafe, UnwindSafe, catch_unwind};
use std::sync::LazyLock;

use actix_web::dev::{HttpServiceFactory, ServiceFactory, ServiceRequest};
use actix_web::web;
use actix_web::{App, Error};
use anyhow::Result;
use blog_client::DtoConstraint;
use rbatis::rbdc::Json;
use serde::Serialize;
use serde::de::DeserializeOwned;
use w_ddd::mapper;

mod article;

pub fn config<T>(app: App<T>) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    app.service(article::hello)
        .service(article::echo)
        .service(article::create_article)
        .route("/hey", web::get().to(article::manual_hello))
}

pub fn map<From, To>(json: web::Json<From>) -> Result<To>
where
    From: 'static + Serialize,
    To: 'static + DeserializeOwned,
{
    mapper::default::<From, To>().map(&json.into_inner())
}

pub fn handle_request<Dto, Entity, F>(json: web::Json<Dto>, handler: F) -> Result<Entity>
where
    Dto: DtoConstraint,
    Entity: DtoConstraint,
    F: Fn(Entity) -> Result<Entity> + UnwindSafe + RefUnwindSafe,
{
    match catch_unwind(move || handler(map(json)?)) {
        Ok(r) => r,
        Err(e) => Err(anyhow::anyhow!("Failed to handle request: {:?}", e)),
    }
}
