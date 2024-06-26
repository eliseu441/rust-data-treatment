
mod services;

use actix_web::{
    App,
    HttpServer,
    middleware::Logger
};

use dotenv::dotenv;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started successfully");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }

    dotenv().ok();
    env_logger::init();




    HttpServer::new(move || {
        App::new()
            .configure(services::config)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run().await
}