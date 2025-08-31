#![allow(unused)]

mod cmd;
mod conf;
mod controller;
mod domain;
mod infra;
mod service;

use actix_web::{App, HttpServer};
use clap::Parser;

use cmd::*;
use config::{Config, File, FileFormat};
use log::{LevelFilter, info};
use simple_logger::SimpleLogger;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_colors(true)
        .env()
        .init()?;

    let cli = Cli::parse();
    match &cli.command {
        Commands::Run { config } => run(config).await?,
        Commands::Hello { name } => hello(name),
        Commands::Hello2(hello2) => hello(&hello2.name),
    }

    Ok(())
}

async fn run(config: &String) -> anyhow::Result<()> {
    conf::init(config)?;
    let config = conf::get_config()?;
    let server = HttpServer::new(|| {
        let mut app = App::new();
        return controller::config(app);
    })
    .bind(config.web.server.to_addr())?;
    info!("Starting web server at {:?}", config.web.server.to_addr());
    server.run().await?;
    Ok(())
}

fn hello(name: &String) {
    println!("Hello, {}!", name);
}
