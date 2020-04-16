#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::config::{Config, IConfig};
use actix_redis::RedisSession;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;

mod api_error;
mod auth;
mod config;
mod db;
mod middlewares;
mod schema;
mod user;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    db::init();

    let mut listenfd = ListenFd::from_env();
    let config = Config {};

    let redis_port = config.get_config_with_key("REDIS_PORT"); //env::var("REDIS_PORT").expect("Redis port not set");
    let redis_host = config.get_config_with_key("REDIS_HOST"); //env::var("REDIS_HOST").expect("Redis host not set");

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(RedisSession::new(
                format!("{}:{}", redis_host, redis_port),
                &[0; 32],
            ))
            .configure(auth::init_routes)
            .configure(user::init_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = config.get_config_with_key("HOST"); //env::var("HOST").expect("Host not set");
            let port = config.get_config_with_key("PORT"); //env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await
}
