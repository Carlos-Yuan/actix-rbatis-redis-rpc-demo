mod auth;
mod config;
mod controller;
mod model;
mod csm;

use std::{net::{SocketAddr}};
use actix_web::{middleware::Logger, App, HttpServer};
use config::Config;
use lazy_static::lazy_static;
use util::redis::RedisConnection;

use crate::auth::Auth;




lazy_static!{
    pub static ref CONF:Config = config::load();
    pub static ref REDIS:RedisConnection =RedisConnection::new(CONF.redis.clone());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fast_log::init(fast_log::config::Config::new().console()).expect("logger init failed");
    log::info!("initiated logging");

   
    log::info!("linking database successful!");

    let server_address: SocketAddr = format!("{}:{}", CONF.ip, CONF.port).parse().expect("couldn't resolve domain");
    HttpServer::new(move || {
        App::new()
            .wrap(Auth)
            .wrap(Logger::default())
            .configure(controller::UserController::config)
            .configure(controller::UserRoleController::config)

    })
    .bind(server_address)?
    .workers(10)
    .run()
    .await
}

