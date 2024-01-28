mod person;
mod recipe;

use std::{env, io};
use actix_web::{App, HttpServer, middleware};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(person::getAll)
            .service(person::get)
            .service(recipe::getAll)
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}